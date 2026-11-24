//! polaros-sensor-probe — one-shot sensor diagnostic.
//!
//! Reads a JSON snapshot of sensor values (from stdin or a file),
//! runs the Lua pipeline `/etc/polaros/sensors/pipeline.lua`,
//! prints the resulting report as JSON.
//!
//! Usage:
//!   polaros-sensor-probe                 # reads /etc/polaros/sensors/last.json
//!   polaros-sensor-probe <file.json>     # reads given file
//!   cat readings.json | polaros-sensor-probe -

use std::process::ExitCode;
use ursus_praefectus as up;

fn read_input(path: &str) -> std::io::Result<String> {
    if path == "-" {
        use std::io::Read;
        let mut buf = String::new();
        std::io::stdin().read_to_string(&mut buf)?;
        Ok(buf)
    } else {
        std::fs::read_to_string(path)
    }
}

fn main() -> ExitCode {
    let path = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "/etc/polaros/sensors/last.json".to_string());

    // 1. Read the input JSON
    let raw = match read_input(&path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("read {path}: {e}");
            return ExitCode::from(2);
        }
    };

    // 2. Parse as JSON
    let ctx: serde_json::Value = match serde_json::from_str(&raw) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("parse {path}: {e}");
            return ExitCode::from(2);
        }
    };

    // 3. Build the Lua sandbox and load the pipeline
    let lua = mlua::Lua::new();
    if let Err(e) = up::lua::setup_sandbox(&lua) {
        eprintln!("sandbox setup failed: {e}");
        return ExitCode::from(1);
    }

    // 4. Run robot.tick(ctx) — same path as the daemon
    match up::lua::call_tick(&lua, ctx) {
        Ok(report) => {
            println!("{}", serde_json::to_string_pretty(&report)
                .unwrap_or_else(|_| "{}".to_string()));
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("lua tick failed: {e}");
            ExitCode::from(1)
        }
    }
}