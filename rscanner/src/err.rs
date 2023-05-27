use serde::Serialize;
use thiserror::Error;

#[derive(Error, Debug, Serialize, PartialEq)]
pub enum APPError {
    #[error("split failed because ip or port format is error")]
    SplitIpPortFailed,
    #[error("port args format is error")]
    PortFormatError,
    #[error("port couldn't be empty")]
    PortIsEmpty,
    #[error("unix ulimits soft limit is bigger than hard limit")]
    ULimitSoftBiggerThanHard,
}

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
