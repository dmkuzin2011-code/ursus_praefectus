//! # Collection sanitization module
//!
//! Provides utilities for cleaning collections by removing unwanted values.
//! Currently supports removing specified values from a mutable vector.

use crate::Ursus;

/// Errors that can occur during collection sanitization.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ClearedError {
    /// The input collection is empty.
    EmptyInput,
}

impl std::fmt::Display for ClearedError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ClearedError::EmptyInput => write!(f, "collection is empty"),
        }
    }
}

impl std::error::Error for ClearedError {}

impl Ursus {
    /// Remove all occurrences of specified values from a collection.
    ///
    /// This function mutates the provided vector in-place, removing any item
    /// whose string representation matches any of the values in delete_values.
    ///
    /// # Errors
    ///
    /// Returns [ClearedError::EmptyInput] if the collection is empty.
    ///
    /// # Example
    ///
    /// ```rust
    /// use ursus_praefectus::Ursus;
    ///
    /// #[tokio::main(flavor = "current_thread")]
    /// async fn main() {
    ///     let mut collection = vec!["apple", "banana", "cherry", "banana"];
    ///     let delete_values = vec!["banana"];
    ///
    ///     Ursus::clear_collection(&mut collection, &delete_values).await.unwrap();
    ///     assert_eq!(collection, vec!["apple", "cherry"]);
    /// }
    /// `
    pub async fn clear_collection<I, T>(
        collection: &mut Vec<I>,
        delete_values: &[T],
    ) -> Result<(), ClearedError>
    where
        I: ToString,
        T: ToString,
    {
        if collection.is_empty() {
            return Err(ClearedError::EmptyInput);
        }
        let to_delete: Vec<String> = delete_values.iter().map(|x| x.to_string()).collect();
        collection.retain(|item| !to_delete.contains(&item.to_string()));
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_clear_collection_basic() {
        let mut collection = vec!["apple", "banana", "cherry", "banana"];
        let delete_values = vec!["banana"];
        assert!(Ursus::clear_collection(&mut collection, &delete_values).await.is_ok());
        assert_eq!(collection, vec!["apple", "cherry"]);
    }

    #[tokio::test]
    async fn test_clear_collection_empty_collection() {
        let mut collection: Vec<&str> = vec![];
        let delete_values = vec!["banana"];
        assert_eq!(
            Ursus::clear_collection(&mut collection, &delete_values).await,
            Err(ClearedError::EmptyInput)
        );
    }

    #[tokio::test]
    async fn test_clear_collection_no_matches() {
        let mut collection = vec!["apple", "cherry"];
        let delete_values = vec!["banana"];
        assert!(Ursus::clear_collection(&mut collection, &delete_values).await.is_ok());
        assert_eq!(collection, vec!["apple", "cherry"]);
    }

    #[tokio::test]
    async fn test_clear_collection_all_removed() {
        let mut collection = vec!["banana", "banana"];
        let delete_values = vec!["banana"];
        assert!(Ursus::clear_collection(&mut collection, &delete_values).await.is_ok());
        assert_eq!(collection, Vec::<&str>::new());
    }

    #[test]
    fn test_cleared_error_display() {
        assert_eq!(ClearedError::EmptyInput.to_string(), "collection is empty");
    }
}

