use std::str::FromStr;

use APRSError;
use Timestamp;
use lonlat::{parse_latitude, parse_longitude};

#[derive(PartialEq, Debug, Clone)]
pub struct APRSObject {
    pub name: String,
    pub status: ObjectStatus,
    pub timestamp: Timestamp,
    pub latitude: f32,
    pub longitude: f32,
    pub comment: String,
    pub symbol_table: char,
    pub symbol: char
}

#[derive(PartialEq, Debug, Clone)]
pub enum ObjectStatus {
    Live,
    Killed,
    Invalid(char)
}

impl FromStr for APRSObject {
    type Err = APRSError;

    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        if s.len() < 42 {
            return Err(APRSError::InvalidObject(s.to_owned()));
        }

        let s = &s[1..s.len()];
        let bytes = s.as_bytes();

        let name = &s[..9];

        let status = bytes[9] as char;
        let status =
            if status == '*' { ObjectStatus::Live }
            else if status == '_' { ObjectStatus::Killed }
            else { ObjectStatus::Invalid(status) };

        let timestamp = s[10..17].parse()?;

        let latitude = parse_latitude(&s[17..25])?;
        let longitude = parse_longitude(&s[26..35])?;

        let comment = &s[42..s.len()];

        Ok(APRSObject{
            name: name.to_owned(),
            status: status,
            timestamp: timestamp,
            latitude: latitude,
            longitude: longitude,
            comment: comment.to_string(),
            symbol_table: bytes[25] as char,
            symbol: bytes[35] as char,
        })
    }
}