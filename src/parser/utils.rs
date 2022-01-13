use std::str::FromStr;

pub(crate) trait StrParts<'a> {
    fn split_line(self) -> Vec<&'a str>;
}

impl<'a> StrParts<'a> for &'a str {
    fn split_line(self) -> Vec<&'a str> {
        self.split_whitespace()
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .map_while(|s| (!s.starts_with('#')).then(|| s))
            // .map_while(|s| (!s.starts_with('#')).then_some(s)); currently still unstable (https://github.com/rust-lang/rust/issues/80967)
            .collect()
    }
}

pub(crate) trait ConvertVec {
    fn convert_vec<T, G>(self) -> Result<Vec<T>, G>
    where
        T: FromStr<Err = G>;
}

impl ConvertVec for Vec<&str> {
    fn convert_vec<T, G>(self) -> Result<Vec<T>, G>
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
            vec!["1.0", "2.0", "-3.0"].convert_vec::<f64, _>(),
            Ok(vec![1.0, 2.0, -3.0])
        );
        assert_eq!(
            vec!["1", "2", "3"].convert_vec::<u8, _>(),
            Ok(vec![1u8, 2u8, 3u8])
        );
        assert_eq!(
            vec!["1", "2", "3.0"]
                .convert_vec::<u8, _>()
                .unwrap_err()
                .kind(),
            &IntErrorKind::InvalidDigit
        );
    }

    #[test]
    fn split_convert() {
        assert_eq!(
            " 1    2 3   ".split_line().convert_vec::<f64, _>(),
            Ok(vec![1.0, 2.0, 3.0])
        );
        assert_eq!(
            "  1 2    	3".split_line().convert_vec::<f32, _>(),
            Ok(vec![1.0, 2.0, 3.0])
        );
    }
}
