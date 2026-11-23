use std::collections::HashMap;
use std::io;
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};
use std::time::{Duration, Instant};

#[derive(Debug)]
pub enum BashError {
    Io(io::Error),
    NonZeroExit { code: Option<i32>, stderr: String },
    Timeout(Duration),
    ScriptNotFound(PathBuf),
}

impl std::fmt::Display for BashError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BashError::Io(e) => write!(f, "io: {e}"),
            BashError::NonZeroExit { code, stderr } => {
                write!(f, "exit {:?}: {}", code, stderr.trim())
            }
            BashError::Timeout(d) => write!(f, "timeout after {:?}", d),
            BashError::ScriptNotFound(p) => write!(f, "script not found: {}", p.display()),
        }
    }
}

impl std::error::Error for BashError {}

impl From<io::Error> for BashError {
    fn from(e: io::Error) -> Self { BashError::Io(e) }
}

pub type BashResult<T> = Result<T, BashError>;

#[derive(Debug, Clone)]
pub struct ShellScript {
    path: PathBuf,
    args: Vec<String>,
    env: HashMap<String, String>,
    cwd: Option<PathBuf>,
    timeout: Option<Duration>,
    capture: bool,
}

impl ShellScript {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        Self {
            path: path.as_ref().to_path_buf(),
            args: Vec::new(),
            env: HashMap::new(),
            cwd: None,
            timeout: None,
            capture: true,
        }
    }

    pub fn arg<S: Into<String>>(mut self, a: S) -> Self {
        self.args.push(a.into());
        self
    }

    pub fn args<I, S>(mut self, items: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.args.extend(items.into_iter().map(Into::into));
        self
    }

    pub fn env<K: Into<String>, V: Into<String>>(mut self, k: K, v: V) -> Self {
        self.env.insert(k.into(), v.into());
        self
    }

    pub fn cwd<P: AsRef<Path>>(mut self, p: P) -> Self {
        self.cwd = Some(p.as_ref().to_path_buf());
        self
    }

    pub fn timeout(mut self, d: Duration) -> Self {
        self.timeout = Some(d);
        self
    }

    pub fn capture(mut self, on: bool) -> Self {
        self.capture = on;
        self
    }

    fn build(&self) -> BashResult<Command> {
        if !self.path.exists() {
            return Err(BashError::ScriptNotFound(self.path.clone()));
        }
        let mut cmd = Command::new("bash");
        cmd.arg(&self.path)
            .args(&self.args)
            .envs(&self.env)
            .stdin(Stdio::null());
        if let Some(dir) = &self.cwd {
            cmd.current_dir(dir);
        }
        if self.capture {
            cmd.stdout(Stdio::piped()).stderr(Stdio::piped());
        }
        Ok(cmd)
    }

    pub fn run(&self) -> BashResult<Output> {
        let mut cmd = self.build()?;
        let child = cmd.spawn()?;

        let output = match self.timeout {
            None => child.wait_with_output()?,
            Some(t) => wait_with_timeout(child, t)?,
        };

        if !output.status.success() {
            return Err(BashError::NonZeroExit {
                code: output.status.code(),
                stderr: String::from_utf8_lossy(&output.stderr).into_owned(),
            });
        }
        Ok(output)
    }

    pub fn run_string(&self) -> BashResult<String> {
        let out = self.run()?;
        Ok(String::from_utf8_lossy(&out.stdout).into_owned())
    }

    pub fn run_lines(&self) -> BashResult<Vec<String>> {
        let s = self.run_string()?;
        Ok(s.lines().map(str::to_string).collect())
    }

    pub fn run_status(&self) -> BashResult<()> {
        self.run().map(|_| ())
    }
}

fn wait_with_timeout(
    mut child: std::process::Child,
    timeout: Duration,
) -> BashResult<Output> {
    let start = Instant::now();
    loop {
        match child.try_wait()? {
            Some(_) => return Ok(child.wait_with_output()?),
            None => {
                if start.elapsed() >= timeout {
                    let _ = child.kill();
                    let _ = child.wait();
                    return Err(BashError::Timeout(timeout));
                }
                std::thread::sleep(Duration::from_millis(20));
            }
        }
    }
}

pub fn run_inline(script: &str) -> BashResult<String> {
    let out = Command::new("bash")
        .arg("-c")
        .arg(script)
        .stdin(Stdio::null())
        .output()?;

    if !out.status.success() {
        return Err(BashError::NonZeroExit {
            code: out.status.code(),
            stderr: String::from_utf8_lossy(&out.stderr).into_owned(),
        });
    }
    Ok(String::from_utf8_lossy(&out.stdout).into_owned())
}

pub fn run_inline_status(script: &str) -> BashResult<()> {
    run_inline(script).map(|_| ())
}

pub fn bash_available() -> bool {
    Command::new("bash")
        .arg("--version")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io::Write;

    fn tmp_script(name: &str, body: &str) -> PathBuf {
        let dir = std::env::temp_dir();
        let path = dir.join(format!("ursus_test_{name}_{}.sh", std::process::id()));
        let mut f = fs::File::create(&path).unwrap();
        f.write_all(body.as_bytes()).unwrap();
        f.sync_all().unwrap();
        path
    }

    fn cleanup(p: &Path) {
        let _ = fs::remove_file(p);
    }

    #[test]
    fn bash_available_check() {
        if !bash_available() {
            eprintln!("bash not available, skipping");
            return;
        }
        assert!(bash_available());
    }

    #[test]
    fn inline_echo() {
        if !bash_available() { return; }
        let out = run_inline("echo hello").unwrap();
        assert_eq!(out.trim(), "hello");
    }

    #[test]
    fn inline_multi_line() {
        if !bash_available() { return; }
        let out = run_inline("echo a; echo b; echo c").unwrap();
        assert_eq!(out.trim(), "a\nb\nc");
    }

    #[test]
    fn inline_non_zero_exit() {
        if !bash_available() { return; }
        let err = run_inline("echo bad >&2; exit 3").unwrap_err();
        match err {
            BashError::NonZeroExit { code, stderr } => {
                assert_eq!(code, Some(3));
                assert_eq!(stderr.trim(), "bad");
            }
            other => panic!("unexpected: {other:?}"),
        }
    }

    #[test]
    fn inline_status_ok() {
        if !bash_available() { return; }
        run_inline_status("true").unwrap();
        assert!(run_inline_status("false").is_err());
    }

    #[test]
    fn script_runs() {
        if !bash_available() { return; }
        let p = tmp_script("basic", "#!/usr/bin/env bash\necho from-script\n");
        let out = BashScript::new(&p).run_string().unwrap();
        assert_eq!(out.trim(), "from-script");
        cleanup(&p);
    }

    #[test]
    fn script_args() {
        if !bash_available() { return; }
        let p = tmp_script("args", "#!/usr/bin/env bash\necho \"$1-$2\"\n");
        let out = BashScript::new(&p).arg("a").arg("b").run_string().unwrap();
        assert_eq!(out.trim(), "a-b");
        cleanup(&p);
    }

    #[test]
    fn script_many_args() {
        if !bash_available() { return; }
        let p = tmp_script("many", "#!/usr/bin/env bash\nprintf '%s\\n' \"$@\"\n");
        let out = BashScript::new(&p)
            .args(vec!["x", "y", "z"])
            .run_lines()
            .unwrap();
        assert_eq!(out, vec!["x", "y", "z"]);
        cleanup(&p);
    }

    #[test]
    fn script_env() {
        if !bash_available() { return; }
        let p = tmp_script("env", "#!/usr/bin/env bash\necho \"$FOO\"\n");
        let out = BashScript::new(&p).env("FOO", "bar").run_string().unwrap();
        assert_eq!(out.trim(), "bar");
        cleanup(&p);
    }

    #[test]
    fn script_cwd() {
        if !bash_available() { return; }
        let p = tmp_script("cwd", "#!/usr/bin/env bash\npwd\n");
        let out = BashScript::new(&p)
            .cwd(std::env::temp_dir())
            .run_string()
            .unwrap();
        assert_eq!(
            fs::canonicalize(out.trim()).unwrap(),
            fs::canonicalize(std::env::temp_dir()).unwrap()
        );
        cleanup(&p);
    }

    #[test]
    fn script_non_zero_exit() {
        if !bash_available() { return; }
        let p = tmp_script("fail", "#!/usr/bin/env bash\necho oops >&2\nexit 7\n");
        let err = BashScript::new(&p).run_status().unwrap_err();
        match err {
            BashError::NonZeroExit { code, stderr } => {
                assert_eq!(code, Some(7));
                assert_eq!(stderr.trim(), "oops");
            }
            other => panic!("unexpected: {other:?}"),
        }
        cleanup(&p);
    }

    #[test]
    fn script_not_found() {
        let err = BashScript::new("/nonexistent/ursus/xx.sh").run().unwrap_err();
        assert!(matches!(err, BashError::ScriptNotFound(_)));
    }

    #[test]
    fn script_timeout() {
        if !bash_available() { return; }
        let p = tmp_script("slow", "#!/usr/bin/env bash\nsleep 5\n");
        let start = Instant::now();
        let err = BashScript::new(&p)
            .timeout(Duration::from_millis(150))
            .run()
            .unwrap_err();
        assert!(matches!(err, BashError::Timeout(_)));
        assert!(start.elapsed() < Duration::from_secs(2));
        cleanup(&p);
    }

    #[test]
    fn script_capture_off_does_not_capture() {
        if !bash_available() { return; }
        let p = tmp_script("nocap", "#!/usr/bin/env bash\necho quiet\n");
        let out = BashScript::new(&p).capture(false).run().unwrap();
        assert!(out.stdout.is_empty());
        cleanup(&p);
    }

    #[test]
    fn error_display() {
        let e = BashError::NonZeroExit {
            code: Some(2),
            stderr: "boom\n".into(),
        };
        assert_eq!(e.to_string(), "exit Some(2): boom");
    }
}