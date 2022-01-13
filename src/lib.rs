pub mod document;
pub mod geometry;
pub(crate) mod parser;

// TODO: verify errors worxing
// TODO: cleanup
// TODO: verify actual counts with header counts

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
