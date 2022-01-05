use strum_macros::{Display, EnumString};

#[derive(Debug, Display, EnumString, Clone, Copy, PartialEq)]
pub enum AuthType {
    #[strum(ascii_case_insensitive)]
    Basic,
    #[strum(ascii_case_insensitive)]
    Bearer
}

#[cfg(test)]
mod tests {
    use crate::AuthType;
    use std::str::FromStr;

    #[test]
    fn test_from_ascii() {
        let test_string = "bearer";
        let auth_type = AuthType::from_str(test_string);
        assert_eq!(AuthType::Bearer, auth_type.unwrap())
    }
}