use snafu::prelude::*;
use snafu::Location;
use std::num::ParseIntError;

#[derive(Snafu, Debug)]
#[snafu(visibility(pub))]
pub enum APPError {
    #[snafu(display("split failed because ip or port format is error {}", value))]
    SplitIpPortFailed { location: Location, value: String },

    #[snafu(display("port args format is error {}", value))]
    PortFormat { location: Location, value: String },

    #[snafu(display("port value parse failed {}", source))]
    PortParse {
        location: Location,
        source: ParseIntError,
        value: String,
    },

    #[snafu(display("port couldn't be empty"))]
    PortIsEmpty,

    #[snafu(display("unix ulimits soft limit is bigger than hard limit"))]
    ULimitSoftBiggerThanHard,

    #[snafu(display("Option value is None"))]
    OptionEmpty { location: Location },

    #[snafu(display("common io error {}", source))]
    CommonIo {
        location: Location,
        source: std::io::Error,
    },
}

pub type Result<T> = std::result::Result<T, APPError>;
