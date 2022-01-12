pub mod document;
pub mod geometry;
pub(crate) mod parser;

// TODO: integration tests
// TODO: error handling
// TODO: think about removing thiserror
// TODO: cleanup
// TODO: verify actual counts with header counts
// TODO: unit tests

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
