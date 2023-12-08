use serde::Serialize;
use snafu::Location;
use snafu::prelude::*;

#[derive(Snafu, Debug, Serialize, PartialEq)]
#[snafu(visibility(pub))]
pub enum APPError {
    #[snafu(display("split failed because ip or port format is error {}", value))]
    SplitIpPortFailed {
        location: Location,
        value: String
    },

    #[snafu(display("port args format is error {}", value))]
    PortFormat {
        location: Location,
        value: String,
    },

    #[snafu(display("port couldn't be empty"))]
    PortIsEmpty,

    #[snafu(display("unix ulimits soft limit is bigger than hard limit"))]
    ULimitSoftBiggerThanHard,

    #[snafu(display("Option value is None"))]
    OptionEmpty {
        location: Location
    },

    #[snafu(display("common io error {}", source))]
    CommonIo {
        location: Location,
        source: std::io::Error,
    },
}

pub type Result<T> = std::result::Result<T, APPError>;

#[cfg(test)]
mod tests {
    use crate::err::APPError;

    #[test]
    fn error_response() {
        for i in 0..3 {
            println!("{:?}", result_with_value(i));
        }
    }

    fn result_with_value(value: u8) -> anyhow::Result<()> {
        if value.eq(&1) {
            return Err(APPError::SplitIpPortFailed.into());
        }
        Ok(())
    }
}
