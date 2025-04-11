use std::str::FromStr;

use crate::error::SelectorError;

/// contract function selector representation.
/// typically used when we want to call a contract function.
///
/// this is will be the first 4 bytes of the calldata.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Selector(String);

impl ToString for Selector {
    fn to_string(&self) -> String {
        self.0.clone()
    }
}

impl FromStr for Selector {
    type Err = crate::error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_raw(s)
    }
}

impl Selector {
    /// try to build a function selector from a string.
    pub fn from_raw(selector: &str) -> Result<Self, crate::error::Error> {
        // strip prefix if any
        let selector = selector.trim_start_matches("0x");
        let bytes = hex::decode(selector).map_err(|_| SelectorError::NotHex)?;

        match bytes.len() {
            4 => Ok(Self(String::from(selector))),
            _ => Err(SelectorError::InvalidLength)?,
        }
    }

    /// return the selector as bytes.
    pub fn encoded(&self) -> Vec<u8> {
        hex::decode(&self.0).unwrap()
    }

    //  TODO
    // pub fn new_with_name(name:&str)->Self{

    // }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_selector_from_str() {
        let selector = Selector::from_str("0xcfdd9aa2");

        match selector {
            Ok(_e) => assert!(true),
            Err(e) => panic!("{:?}", e),
        }
    }

    #[test]
    fn test_selector_from_raw() {
        let selector = Selector::from_raw("0xcfdd9aa2");
        match selector {
            Ok(_e) => assert!(true),
            Err(e) => panic!("{:?}", e),
        }
    }
}
