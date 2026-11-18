//! # Collection sanitization module
//!
//! Provides utilities for cleaning collections by removing unwanted values.
//! Currently supports removing specified values from a mutable vector.

use std::collections::HashSet;

use crate::Ursus;
use crate::specific_errors::ValidationError;

impl Ursus {
    /// Remove all occurrences of specified values from a collection.
    ///
    /// This function mutates the provided vector in-place, removing any item
    /// whose string representation matches any of the values in delete_values.
    ///
    /// # Errors
    ///
    /// Returns [`ValidationError::EmptyInput`] if the collection is empty.
    ///
    /// # Example
    ///
    /// ```rust
    /// use ursus_praefectus::Ursus;
    ///
    /// fn main() {
    ///     let mut collection = vec!["apple", "banana", "cherry", "banana"];
    ///     let delete_values = vec!["banana"];
    ///
    ///     Ursus::clear_collection(&mut collection, &delete_values).unwrap();
    ///     assert_eq!(collection, vec!["apple", "cherry"]);
    /// }
    /// ```
    pub fn clear_collection<I, T>(
        collection: &mut Vec<I>,
        delete_values: &[T],
    ) -> Result<(), ValidationError>
    where
        I: AsRef<str>,
        T: AsRef<str>,
    {
        if collection.is_empty() {
            return Err(ValidationError::EmptyInput);
        }
        let to_delete: HashSet<&str> = delete_values.iter().map(|x| x.as_ref()).collect();
        collection.retain(|item| !to_delete.contains(&item.as_ref()));
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clear_collection_basic() {
        let mut collection = vec!["apple", "banana", "cherry", "banana"];
        let delete_values = vec!["banana"];
        assert!(Ursus::clear_collection(&mut collection, &delete_values).is_ok());
        assert_eq!(collection, vec!["apple", "cherry"]);
    }

    #[test]
    fn test_clear_collection_empty_collection() {
        let mut collection: Vec<&str> = vec![];
        let delete_values = vec!["banana"];
        assert_eq!(
            Ursus::clear_collection(&mut collection, &delete_values),
            Err(ValidationError::EmptyInput)
        );
    }

    #[test]
    fn test_clear_collection_no_matches() {
        let mut collection = vec!["apple", "cherry"];
        let delete_values = vec!["banana"];
        assert!(Ursus::clear_collection(&mut collection, &delete_values).is_ok());
        assert_eq!(collection, vec!["apple", "cherry"]);
    }

    #[test]
    fn test_clear_collection_all_removed() {
        let mut collection = vec!["banana", "banana"];
        let delete_values = vec!["banana"];
        assert!(Ursus::clear_collection(&mut collection, &delete_values).is_ok());
        assert_eq!(collection, Vec::<&str>::new());
    }
}

