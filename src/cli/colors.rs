#[cfg(feature = "cli-colors")]
mod imp {
    use std::io::Write;
    use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

    fn choice() -> ColorChoice {
        if std::env::var_os("NO_COLOR").is_some() {
            ColorChoice::Never
        } else if std::env::var_os("CLICOLOR_FORCE").is_some() {
            ColorChoice::Always
        } else {
            ColorChoice::Auto
        }
    }

    fn print(msg: &str, color: Color, bold: bool, stderr: bool) {
        let mut spec = ColorSpec::new();
        spec.set_fg(Some(color)).set_bold(bold);

        if stderr {
            let mut out = StandardStream::stderr(choice());
            let _ = out.set_color(&spec);
            let _ = writeln!(out, "{msg}");
            let _ = out.reset();
        } else {
            let mut out = StandardStream::stdout(choice());
            let _ = out.set_color(&spec);
            let _ = writeln!(out, "{msg}");
            let _ = out.reset();
        }
    }

    pub fn success(msg: &str) { print(msg, Color::Green, false, false); }
    pub fn error(msg: &str)   { print(msg, Color::Red,   false, true);  }
    pub fn warning(msg: &str) { print(msg, Color::Yellow,false, true);  }
    pub fn info(msg: &str)    { print(msg, Color::Cyan,  false, false); }
    pub fn debug(msg: &str)   { print(msg, Color::Magenta,false,true);  }
    pub fn bold(msg: &str)    { print(msg, Color::White, true,  false); }
    pub fn dim(msg: &str)     { print(msg, Color::Rgb(128,128,128), false, false); }

    pub fn colored(msg: &str, color: Color, bold: bool) {
        print(msg, color, bold, false);
    }

    pub fn is_enabled() -> bool {
        !matches!(choice(), ColorChoice::Never)
    }
}

#[cfg(not(feature = "cli-colors"))]
mod imp {
    pub fn success(msg: &str) { println!("[OK] {msg}"); }
    pub fn error(msg: &str)   { eprintln!("[ERROR] {msg}"); }
    pub fn warning(msg: &str) { eprintln!("[WARN] {msg}"); }
    pub fn info(msg: &str)    { println!("[INFO] {msg}"); }
    pub fn debug(msg: &str)   { eprintln!("[DEBUG] {msg}"); }
    pub fn bold(msg: &str)    { println!("{msg}"); }
    pub fn dim(msg: &str)     { println!("{msg}"); }

    pub fn colored(msg: &str, _color: (), _bold: bool) { println!("{msg}"); }

    pub fn is_enabled() -> bool { false }
}

pub use imp::*;