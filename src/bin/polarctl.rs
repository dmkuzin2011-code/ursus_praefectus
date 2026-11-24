//! polarctl — CLI for validation and Lua checks.

use std::process::ExitCode;
use ursus_praefectus as up;

fn usage() -> ! {
    eprintln!(
        "usage:\n  \
         polarctl email <addr>\n  \
         polarctl url   <url>\n  \
         polarctl ip    <addr>\n  \
         polarctl phone <number>\n  \
         polarctl lua   <script.lua>"
    );
    std::process::exit(2);
}

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.len() < 2 { usage(); }
    let (cmd, value) = (args[0].as_str(), args[1].as_str());

    let ok = match cmd {
        "email" => up::email::is_valid(value),
        "url"   => up::url::is_valid(value),
        "ip"    => up::ip::is_valid(value),
        "phone" => up::phone::is_valid(value),
        "lua" => {
            match std::fs::read_to_string(value) {
                Ok(src) => {
                    let lua = mlua::Lua::new();
                    if let Err(e) = up::lua::setup_sandbox(&lua) {
                        eprintln!("sandbox setup failed: {e}");
                        return ExitCode::from(1);
                    }
                    match lua.load(&src).set_name(value).exec() {
                        Ok(()) => true,
                        Err(e) => { eprintln!("lua error: {e}"); false }
                    }
                }
                Err(e) => { eprintln!("read {value}: {e}"); false }
            }
        }
        _ => usage(),
    };

    if ok {
        println!("\u{2705} {cmd} valid: {value}");
        ExitCode::SUCCESS
    } else {
        eprintln!("\u{274c} {cmd} invalid: {value}");
        ExitCode::from(1)
    }
}