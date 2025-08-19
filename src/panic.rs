//! Custom panic hook, derived from the code for the [panic crate](https://github.com/exact-labs/panic).

use std::{borrow::Cow, collections::HashMap, error::Error, fmt::Write as FmtWrite, fs::File, io::{Result as IoResult, Write}, mem, panic::PanicHookInfo, path::{Path, PathBuf}};

pub use anstyle::AnsiColor as Color;
use backtrace::Backtrace;
use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Serialize, Clone, Copy)]
pub enum Method {
    Panic,
}

#[derive(Debug, Serialize)]
pub struct Report {
    name: String,
    operating_system: String,
    crate_version: String,
    explanation: String,
    cause: String,
    method: Method,
    backtrace: String,
}

impl Report {
    pub fn new(
        name: &str,
        version: &str,
        method: Method,
        explanation: String,
        cause: String,
    ) -> Self {
        let operating_system = os_info::get().to_string();

        const SKIP_FRAMES_NUM: usize = 8;
        const HEX_WIDTH: usize = mem::size_of::<usize>() + 2;
        const NEXT_SYMBOL_PADDING: usize = HEX_WIDTH + 6;

        let mut backtrace = String::new();
        for (idx, frame) in Backtrace::new()
            .frames()
            .iter()
            .skip(SKIP_FRAMES_NUM)
            .enumerate()
        {
            let ip = frame.ip();
            let _ = write!(backtrace, "\n{idx:4}: {ip:HEX_WIDTH$?}");

            let symbols = frame.symbols();
            if symbols.is_empty() {
                let _ = write!(backtrace, " - <unresolved>");
                continue;
            }

            for (idx, symbol) in symbols.iter().enumerate() {
                if idx != 0 {
                    let _ = write!(backtrace, "\n{:1$}", "", NEXT_SYMBOL_PADDING);
                }

                if let Some(name) = symbol.name() {
                    let _ = write!(backtrace, " - {name}");
                } else {
                    let _ = write!(backtrace, " - <unknown>");
                }

                if let (Some(file), Some(line)) = (symbol.filename(), symbol.lineno()) {
                    let _ = write!(
                        backtrace,
                        "\n{:3$}at {}:{}",
                        "",
                        file.display(),
                        line,
                        NEXT_SYMBOL_PADDING
                    );
                }
            }
        }

        Self {
            crate_version: version.into(),
            name: name.into(),
            operating_system,
            method,
            explanation,
            cause,
            backtrace,
        }
    }

    pub fn serialize(&self) -> Option<String> {
        toml::to_string_pretty(&self).ok()
    }

    pub fn persist(&self) -> Result<PathBuf, Box<dyn Error + 'static>> {
        let uuid = Uuid::new_v4().hyphenated().to_string();
        let tmp_dir = dirs::cache_dir().unwrap();
        let file_name = format!("report-{}.toml", &uuid);
        let file_path = Path::new(&tmp_dir).join(file_name);
        let mut file = File::create(&file_path)?;
        let toml = self.serialize().unwrap();
        file.write_all(toml.as_bytes())?;
        Ok(file_path)
    }
}

struct Writer<'w> {
    meta: &'w Metadata,
    table: HashMap<&'w str, &'w str>,
    pub(crate) buffer: &'w mut dyn std::io::Write,
}

pub struct Metadata {
    pub name: Cow<'static, str>,
    pub version: Cow<'static, str>,
    pub repository: Cow<'static, str>,
}

#[macro_export]
macro_rules! setup_panic {
    (@field_arm colors $value:expr, $meta:expr) => {
        $meta.messages.head.1 = $value.0;
        $meta.messages.body.1 = $value.1;
        $meta.messages.footer.1 = $value.2;
    };
    (@field_arm $field:ident $value:expr, $meta:expr) => {
        let value: Cow<'static, str> = $value.into();
        $meta.messages.$field.0 = Some(value).filter(|val| !val.is_empty());
    };
    (
        $(name: $name:expr,)?
        $(short_name: $short_name:expr,)?
        $(version: $version:expr,)?
        $(repository: $repository:expr,)?
        $(messages: {
            $(colors: $colors:expr,)?
            $(head: $head:expr,)?
            $(body: $body:expr,)?
            $(footer: $footer:expr)?
        })?
    ) => {
        use std::{panic::{self, PanicHookInfo}};
        use $crate::panic::{handle_dump, print_msg, Metadata};

        // Always attempt to restore terminal
        ratatui::restore();

        let meta = Metadata {
            name: env!("CARGO_PKG_NAME").into(),
            version: env!("CARGO_PKG_VERSION").into(),
            repository: env!("CARGO_PKG_REPOSITORY").into(),
        };

        $(meta.name=$name.into();)?
        $(meta.short_name=$short_name.into();)?
        $(meta.version=$version.into();)?
        $(meta.repository=$repository.into();)?

        $(
            $($crate::setup_panic!(@field_arm head $head, meta);)?
            $($crate::setup_panic!(@field_arm body $body, meta);)?
            $($crate::setup_panic!(@field_arm footer $footer, meta);)?
            $($crate::setup_panic!(@field_arm colors $colors, meta);)?
        )?

        match $crate::panic::PanicStyle::default() {
            $crate::panic::PanicStyle::Debug => {}
            $crate::panic::PanicStyle::Human => {
                panic::set_hook(Box::new(move |info: &PanicHookInfo| {
                    let message = match (
                        info.payload().downcast_ref::<&str>(),
                        info.payload().downcast_ref::<String>(),
                    ) {
                        (Some(s), _) => Some((*s).to_owned()),
                        (_, Some(s)) => Some(s.to_string()),
                        (None, None) => None,
                    };

                    let cause = match message {
                        Some(m) => m,
                        None => "Unknown".into(),
                    };

                    let file_path = handle_dump(&meta, info, &cause);
                    print_msg(file_path, &meta, cause).expect("human-panic: printing error message to console failed");
                }));
            }
        }
    };
}

#[cfg_attr(debug_assertions, derive(Default))]
#[derive(Copy, Clone, PartialEq, Eq)]
#[allow(dead_code)]
pub enum PanicStyle {
    #[cfg_attr(debug_assertions, default)]
    Debug,
    Human,
}

#[cfg(not(debug_assertions))]
impl Default for PanicStyle {
    fn default() -> Self {
        match ::std::env::var("RUST_BACKTRACE") {
            Ok(_) => PanicStyle::Debug,
            Err(_) => PanicStyle::Human,
        }
    }
}

pub fn print_msg<P: AsRef<Path>>(
    file_path: Option<P>,
    meta: &Metadata,
    cause: String,
) -> IoResult<()> {
    let stderr = anstream::stderr();
    let mut stderr = stderr.lock();
    let mut writer = Writer::new(&mut stderr, file_path, meta);

    writer.head()?;
    writer.body(cause)?;

    Ok(())
}

impl<'w> Writer<'w> {
    fn new<P: AsRef<Path>>(
        buffer: &'w mut impl std::io::Write,
        file_path: Option<P>,
        meta: &'w Metadata,
    ) -> Self {
        let mut table: HashMap<&str, &str> = HashMap::new();

        let file_path = match file_path {
            Some(fp) => format!("{}", fp.as_ref().display()),
            None => "<Failed to store file to disk>".to_owned(),
        };

        table.insert("file_path", Box::leak(Box::new(file_path)));

        Self {
            buffer,
            meta,
            table,
        }
    }

    fn head(&mut self) -> IoResult<()> {
        let (name, version) = (&self.meta.name, &self.meta.version);

        write!(self.buffer, "{}", Color::Red.render_fg())?;
        writeln!(
            self.buffer,
            "\n{name} v{version} had a problem and crashed."
        )?;
        write!(self.buffer, "{}", anstyle::Reset.render())?;

        Ok(())
    }

    fn body(&mut self, cause: String) -> IoResult<()> {
        let repository = &self.meta.repository;
        let file_path = self.table.get("file_path").unwrap();

        write!(self.buffer, "{}", Color::Red.render_fg())?;
        writeln!(self.buffer, "\nCause: {cause}\n")?;
        write!(self.buffer, "{}", anstyle::Reset.render())?;

        writeln!(
            self.buffer,
            "A report file has been generated at {file_path}.\n\
             Please submit an issue including the report at {repository}/issues.",
        )?;

        Ok(())
    }
}

pub fn handle_dump(meta: &Metadata, panic_info: &PanicHookInfo, cause: &str) -> Option<PathBuf> {
    let mut expl = String::new();

    match panic_info.location() {
        Some(location) => writeln!(
            expl,
            "Panic occurred in file '{}' at line {}",
            location.file(),
            location.line()
        )
        .unwrap(),
        None => expl.push_str("Panic location unknown.\n"),
    }

    let report = Report::new(
        &meta.name,
        &meta.version,
        Method::Panic,
        expl,
        cause.to_owned(),
    );

    report
        .persist()
        .inspect_err(|_| eprintln!("{}", report.serialize().unwrap()))
        .ok()
}
