use std::str::FromStr;

use APRSError;
use Timestamp;

#[derive(PartialEq, Debug, Clone)]
pub struct APRSStatusReport {
    pub timestamp: Option<Timestamp>,
    pub report: String
}

impl FromStr for APRSStatusReport {
    type Err = APRSError;

    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        let s = &s[1..s.len()];

        // TODO: Not Complete
        Ok(APRSStatusReport{
            timestamp: None,
            report: s.to_owned()
        })
    }
}