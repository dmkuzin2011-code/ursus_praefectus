use std::process::{Command, Stdio};
use ursus_praefectus::cli::colors;

fn run_one(name: &str, program: &str, args: &[&str]) -> std::io::Result<bool> {
    colors::bold(&format!("==> {name}: {program} {}", args.join(" ")));

    let output = Command::new(program)
        .args(args)
        .stdin(Stdio::null())
        .output()?;

    if output.status.success() {
        colors::success(&format!("[{name}] exited 0"));
    } else {
        colors::error(&format!("[{name}] exited {:?}", output.status.code()));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    for line in stdout.lines() {
        colors::info(&format!("[{name}] {line}"));
    }

    let stderr = String::from_utf8_lossy(&output.stderr);
    for line in stderr.lines() {
        colors::warning(&format!("[{name}] {line}"));
    }

    Ok(output.status.success())
}

fn main() {
    let jobs = [
        ("echo",     "echo",     &["hello"][..]),
        ("true",     "true",     &[][..]),
        ("false",    "false",    &[][..]),
        ("ls",       "ls",       &["-1", "/"][..]),
    ];

    let mut ok = 0;
    let mut failed = 0;

    for (name, program, args) in jobs {
        match run_one(name, program, args) {
            Ok(true)  => ok += 1,
            Ok(false) => failed += 1,
            Err(e)    => {
                colors::error(&format!("[{name}] spawn failed: {e}"));
                failed += 1;
            }
        }
    }

    colors::bold(&format!("summary: {ok} ok, {failed} failed"));
    if failed > 0 {
        std::process::exit(1);
    }
}