use crate::specific_errors::ValidationError;

pub fn count_occurrences<T, I>(collection: I, target: T) -> Result<usize, ValidationError>
where
    T: AsRef<str>,
    I: IntoIterator<Item = T>,
{
    let mut count = 0;

    let target_str: &str = target.as_ref();

    if target_str.is_empty() {
        return Err(ValidationError::EmptyInput);
    }
    for item in collection {
        let item_str: &str = item.as_ref();
        if item_str == target_str {
            count += 1;
        }
    }
    Ok(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_occurrences_basic() {
        let collection: Vec<&str> = vec!["a", "b", "a", "c", "a"];
        let target: &str = "a";
        assert_eq!(count_occurrences(collection, target), Ok(3));
    }

    #[test]
    fn test_count_occurrences_no_matches() {
        let collection: Vec<&str> = vec!["x", "y", "z"];
        let target: &str = "a";
        assert_eq!(count_occurrences(collection, target), Ok(0));
    }

    #[test]
    fn test_count_occurrences_empty_collection() {
        let collection: Vec<&str> = vec![];
        let target: &str = "a";
        assert_eq!(count_occurrences(collection, target), Ok(0));
    }

    #[test]
    fn test_count_occurrences_empty_target() {
        let collection: Vec<&str> = vec!["a", "b", "c"];
        let target: &str = "";
        assert_eq!(count_occurrences(collection, target), Err(ValidationError::EmptyInput));
    }

    #[test]
    fn test_count_occurrences_empty_item_in_collection() {
        let collection: Vec<&str> = vec!["a", "", "b"];
        let target: &str = "a";
        assert_eq!(count_occurrences(collection, target), Ok(1));
    }

    #[test]
    fn test_count_occurrences_case_sensitive() {
        let collection: Vec<&str> = vec!["A", "a", "A", "a"];
        let target: &str = "a";
        assert_eq!(count_occurrences(collection, target), Ok(2));
    }

    #[test]
    fn test_count_occurrences_all_match() {
        let collection: Vec<&str> = vec!["hello", "hello", "hello"];
        let target: &str = "hello";
        assert_eq!(count_occurrences(collection, target), Ok(3));
    }
}