use std::cmp::max;

use crate::err::APPError;

pub fn get_ulimit_info() -> anyhow::Result<(u64, u64)> {
    let (soft, hard) = rlimit::Resource::NOFILE.get()?;
    tracing::info!("system now ulimit soft:[{soft}] hard[{hard}]");
    Ok((soft, hard))
}

pub fn set_ulimit(soft: u64, hard: u64) -> anyhow::Result<()> {
    tracing::info!("begin change system ulimit");
    let (original_soft, original_hard) = get_ulimit_info()?;
    let changed_soft = max(soft, original_soft);
    let changed_hard = max(hard, original_hard);
    if soft > hard {
        return Err(APPError::ULimitSoftBiggerThanHard.into());
    }
    rlimit::Resource::NOFILE.set(changed_soft, changed_hard)?;
    get_ulimit_info()?;
    Ok(())
}
