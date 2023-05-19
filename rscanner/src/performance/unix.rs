use std::cmp::max;
use std::process::Command;

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

pub fn set_sysctl_conf() -> anyhow::Result<()> {
    for config in &[
        "net.unix.max_dgram_qlen=100000000",
        // Bigger buffers (to make 40Gb more practical). These are maximums, but the default is unaffected.
        "net.core.wmem_max=268435456",
        "net.core.rmem_max=268435456",
        "net.core.netdev_max_backlog=1000000",
        // Avoids problems with multicast traffic arriving on non-default interfaces
        "net.ipv4.conf.default.rp_filter=0",
        "net.ipv4.conf.all.rp_filter=0",
        // Force IGMP v2 (required by CBF switch)
        "net.ipv4.conf.all.force_igmp_version=2",
        "net.ipv4.conf.default.force_igmp_version=2",
        //  Increase the ARP cache table
        "net.ipv4.neigh.default.gc_thresh3=409600",
        "net.ipv4.neigh.default.gc_thresh2=204800",
        "net.ipv4.neigh.default.gc_thresh1=102400",
        // Increase number of multicast groups permitted
        "net.ipv4.igmp_max_memberships=10240",
    ] {
        let output = Command::new("sysctl").args(["-w", config]).output()?;
        tracing::info!("stdout: {}", String::from_utf8(output.stdout)?);
    }
    Ok(())
}

pub fn improve_limits() -> anyhow::Result<()> {
    let ulimit = 1024 * 1024;
    set_ulimit(ulimit, ulimit)?;
    set_sysctl_conf()?;
    Ok(())
}
