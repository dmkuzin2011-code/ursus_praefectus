use std::env;

#[derive(Debug, Clone, Default)]
pub struct Args {
    items: Vec<String>,
}

impl Args {
    pub fn new() -> Self {
        Self { items: env::args().collect() }
    }

    pub fn from_vec(items: Vec<String>) -> Self {
        Self { items }
    }

    pub fn program(&self) -> Option<&str> {
        self.items.first().map(String::as_str)
    }

    pub fn args(&self) -> &[String] {
        if self.items.is_empty() { &[] } else { &self.items[1..] }
    }

    pub fn len(&self) -> usize {
        self.items.len().saturating_sub(1)
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn get(&self, index: usize) -> Option<&str> {
        self.args().get(index).map(String::as_str)
    }

    pub fn contains(&self, flag: &str) -> bool {
        self.args().iter().any(|a| a == flag)
    }

    pub fn value_of(&self, flag: &str) -> Option<&str> {
        let args = self.args();
        args.iter()
            .position(|a| a == flag)
            .and_then(|i| args.get(i + 1))
            .map(String::as_str)
    }

    pub fn value_of_eq(&self, flag: &str) -> Option<&str> {
        let prefix = format!("{flag}=");
        self.args().iter().find_map(|a| a.strip_prefix(&prefix))
    }

    pub fn iter(&self) -> std::slice::Iter<'_, String> {
        self.args().iter()
    }
}

impl<'a> IntoIterator for &'a Args {
    type Item = &'a String;
    type IntoIter = std::slice::Iter<'a, String>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn args(v: &[&str]) -> Args {
        Args::from_vec(v.iter().map(|s| s.to_string()).collect())
    }

    #[test]
    fn empty_vec_has_no_program_and_no_args() {
        let a = args(&[]);
        assert_eq!(a.program(), None);
        assert_eq!(a.args(), &[] as &[String]);
        assert_eq!(a.len(), 0);
        assert!(a.is_empty());
        assert_eq!(a.get(0), None);
    }

    #[test]
    fn only_program_name() {
        let a = args(&["app"]);
        assert_eq!(a.program(), Some("app"));
        assert_eq!(a.args(), &[] as &[String]);
        assert_eq!(a.len(), 0);
        assert!(a.is_empty());
    }

    #[test]
    fn program_plus_positional() {
        let a = args(&["app", "foo", "bar"]);
        assert_eq!(a.program(), Some("app"));
        assert_eq!(a.len(), 2);
        assert!(!a.is_empty());
        assert_eq!(a.get(0), Some("foo"));
        assert_eq!(a.get(1), Some("bar"));
        assert_eq!(a.get(2), None);
    }

    #[test]
    fn args_slice_excludes_program() {
        let a = args(&["app", "x", "y", "z"]);
        let slice: Vec<&str> = a.args().iter().map(String::as_str).collect();
        assert_eq!(slice, vec!["x", "y", "z"]);
    }

    #[test]
    fn contains_flag() {
        let a = args(&["app", "--verbose", "file.txt"]);
        assert!(a.contains("--verbose"));
        assert!(!a.contains("--quiet"));
    }

    #[test]
    fn value_of_returns_next() {
        let a = args(&["app", "--output", "out.txt"]);
        assert_eq!(a.value_of("--output"), Some("out.txt"));
    }

    #[test]
    fn value_of_missing_flag() {
        let a = args(&["app", "--output", "out.txt"]);
        assert_eq!(a.value_of("--input"), None);
    }

    #[test]
    fn value_of_flag_at_end_has_no_value() {
        let a = args(&["app", "--output"]);
        assert_eq!(a.value_of("--output"), None);
    }

    #[test]
    fn value_of_eq_form() {
        let a = args(&["app", "--output=out.txt"]);
        assert_eq!(a.value_of_eq("--output"), Some("out.txt"));
    }

    #[test]
    fn value_of_eq_missing() {
        let a = args(&["app", "--output", "out.txt"]);
        assert_eq!(a.value_of_eq("--output"), None);
    }

    #[test]
    fn iter_yields_user_args_only() {
        let a = args(&["app", "a", "b", "c"]);
        let collected: Vec<&str> = a.iter().map(String::as_str).collect();
        assert_eq!(collected, vec!["a", "b", "c"]);
    }

    #[test]
    fn into_iter_borrowed() {
        let a = args(&["app", "a", "b"]);
        let mut collected = Vec::new();
        for item in &a {
            collected.push(item.as_str());
        }
        assert_eq!(collected, vec!["a", "b"]);
    }

    #[test]
    fn program_none_when_empty() {
        let a = Args::default();
        assert_eq!(a.program(), None);
        assert!(a.is_empty());
    }
}