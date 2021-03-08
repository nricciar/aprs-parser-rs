use std::str::FromStr;

use APRSError;
use Timestamp;
use lonlat::{parse_latitude, parse_longitude};

#[derive(PartialEq, Debug, Clone)]
pub struct APRSTelemetry {
    pub latitude: f32,
    pub longitude: f32,
    pub comment: String,
    pub symbol_table: char,
    pub symbol: char,
}

impl FromStr for APRSTelemetry {
    type Err = APRSError;

    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        let s = &s[1..s.len()];
        let compressed = &s[..8];   
        let symbol_table = compressed.chars().nth(7);
        let symbol = compressed.chars().nth(6);

        println!("incoming: {}", s);
        println!("symbol table: {:?} -- symbol: {:?}", symbol_table, symbol);
        println!("compressed: {}", compressed);

        Err(APRSError::UnsupportedPositionFormat(s.to_owned()))
    }
}