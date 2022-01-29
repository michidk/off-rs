use std::str::FromStr;

/// A trait defining the interface for spliting a string into a vector of strings.
pub(crate) trait StrParts<'a> {
    fn split_line(self) -> Vec<&'a str>;
}

impl<'a> StrParts<'a> for &'a str {
    /// Splits a string into a vector of strings at whitespaces and ignores comments.
    fn split_line(self) -> Vec<&'a str> {
        self.split_whitespace()
            .map_while(|s| (!s.starts_with('#')).then(|| s))
            // .map_while(|s| (!s.starts_with('#')).then_some(s)); currently still unstable (https://github.com/rust-lang/rust/issues/80967)
            .collect()
    }
}

/// A trait defining the interface for converting a string vector to a different type.
pub(crate) trait ConvertVec {
    fn parse_string_to<T, G>(self) -> Result<Vec<T>, G>
    where
        T: FromStr<Err = G>;
}

impl ConvertVec for Vec<&str> {
    /// Converts a string vector to a vector of different types.
    fn parse_string_to<T, G>(self) -> Result<Vec<T>, G>
    where
        T: FromStr<Err = G>,
    {
        self.into_iter()
            .map(FromStr::from_str)
            .collect::<Result<Vec<T>, G>>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::num::IntErrorKind;

    #[test]
    fn split_line() {
        assert_eq!("".split_line(), Vec::<&str>::new());
        assert_eq!("1 2 3".split_line(), vec!["1", "2", "3"]);
        assert_eq!("1   2      3.0".split_line(), vec!["1", "2", "3.0"]);
    }

    #[test]
    fn convert_vec() {
        assert_eq!(
            vec!["1.0", "2.0", "-3.0"].parse_string_to::<f64, _>(),
            Ok(vec![1.0, 2.0, -3.0])
        );
        assert_eq!(
            vec!["1", "2", "3"].parse_string_to::<u8, _>(),
            Ok(vec![1u8, 2u8, 3u8])
        );
        assert_eq!(
            vec!["1", "2", "3.0"]
                .parse_string_to::<u8, _>()
                .unwrap_err()
                .kind(),
            &IntErrorKind::InvalidDigit
        );
    }

    #[test]
    fn split_convert() {
        assert_eq!(
            " 1    2 3   ".split_line().parse_string_to::<f64, _>(),
            Ok(vec![1.0, 2.0, 3.0])
        );
        assert_eq!(
            "  1 2    	3".split_line().parse_string_to::<f32, _>(),
            Ok(vec![1.0, 2.0, 3.0])
        );
    }
}
