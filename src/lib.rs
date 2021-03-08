//! [APRS] message parser for [Rust]
//!
//! [APRS]: http://www.aprs.org/
//! [Rust]: https://www.rust-lang.org/
//!
//! # Usage
//!
//! ```rust
//! extern crate aprs_parser;
//!
//! fn main() {
//!     let result = aprs_parser::parse(
//!         r"ICA3D17F2>APRS,qAS,dl4mea:/074849h4821.61N\01224.49E^322/103/A=003054"
//!     );
//!
//!     println!("{:#?}", result);
//!
//!     // Ok(
//!     //     APRSMessage {
//!     //         from: Callsign {
//!     //             call: "ICA3D17F2",
//!     //             ssid: None
//!     //         },
//!     //         to: Callsign {
//!     //             call: "APRS",
//!     //             ssid: None
//!     //         },
//!     //         via: [
//!     //             Callsign {
//!     //                 call: "qAS",
//!     //                 ssid: None
//!     //             },
//!     //             Callsign {
//!     //                 call: "dl4mea",
//!     //                 ssid: None
//!     //             }
//!     //         ],
//!     //         data: Position(
//!     //             APRSPosition {
//!     //                 timestamp: Some(
//!     //                     HHMMSS(
//!     //                         7,
//!     //                         48,
//!     //                         49
//!     //                     )
//!     //                 ),
//!     //                 latitude: 48.360165,
//!     //                 longitude: 12.408166,
//!     //                 comment: "322/103/A=003054"
//!     //             }
//!     //         )
//!     //     }
//!     // )
//! }
//! ```

extern crate failure;
#[macro_use] extern crate failure_derive;

#[cfg(test)] #[macro_use] extern crate approx;

mod error;
mod callsign;
mod lonlat;
mod message;
mod position;
mod telemetry;
mod timestamp;
mod object;
mod status_report;

use std::str::FromStr;

pub use error::APRSError;
pub use callsign::Callsign;
pub use message::{APRSMessage, APRSData};
pub use position::APRSPosition;
pub use telemetry::APRSTelemetry;
pub use object::APRSObject;
pub use status_report::APRSStatusReport;
pub use timestamp::Timestamp;

pub fn parse(s: &str) -> Result<APRSMessage, APRSError> {
    APRSMessage::from_str(s)
}
