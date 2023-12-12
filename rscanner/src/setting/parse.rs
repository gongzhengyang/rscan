use std::net::Ipv4Addr;
use std::str::FromStr;
use std::sync::Arc;

use ipnetwork::Ipv4Network;
use snafu::{IntoError, NoneError, ResultExt};

use crate::err::{PortFormatSnafu, PortParseSnafu, Result};

/// parse input like
/// single ip `192.168.1.1`
/// ip with cidr `192.168.1.0/12`
/// combines all formats with comma separate `192.168.1.1,192.168.1.0,192.168.2.1/24`
pub fn parse_hosts(value: &str) -> anyhow::Result<Arc<Vec<Ipv4Addr>>> {
    let splits = value.split(',');
    let mut full_values = vec![];
    for i in splits {
        let results = Ipv4Network::from_str(i)?
            .into_iter()
            .collect::<Vec<Ipv4Addr>>();
        full_values.extend(results);
    }
    Ok(Arc::new(full_values))
}

pub fn parse_ports(input: &str) -> anyhow::Result<Arc<Vec<u16>>> {
    if input.eq("") {
        return Ok(Arc::new(vec![]));
    }
    let mut ports = vec![];
    for i in input.split(',') {
        if i.contains('-') {
            ports.extend(parse_ports_range(i)?);
        } else {
            ports.push(i.parse::<u16>()?);
        }
    }
    Ok(Arc::new(ports))
}

pub fn parse_ports_range(input: &str) -> Result<Vec<u16>> {
    let range = input.split('-').collect::<Vec<&str>>();
    if let [start, end] = range.as_slice() {
        let parsed_start = start.parse::<u16>().context(PortParseSnafu {
            value: start.to_owned(),
        })?;
        let parsed_end = end.parse::<u16>().context(PortParseSnafu {
            value: end.to_owned(),
        })?;
        let result = (parsed_start..parsed_end).collect::<Vec<u16>>();
        Ok(result)
    } else {
        Err(PortFormatSnafu { value: input }.into_error(NoneError))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn multi_ipv4_values() {
        let results = parse_hosts("192.168.1.1,192.168.1.2,1.1.1.1/24").unwrap();
        let mut compare = vec![];
        let first = u32::from(Ipv4Addr::from_str("192.168.1.1").unwrap());
        compare.push(Ipv4Addr::from(first));
        compare.push(Ipv4Addr::from(first + 1));
        let cidr_start = u32::from(Ipv4Addr::from_str("1.1.1.0").unwrap());
        compare.extend(
            (0..256)
                .map(|x| Ipv4Addr::from(x + cidr_start))
                .collect::<Vec<Ipv4Addr>>(),
        );
        assert_eq!(results, Arc::new(compare));
    }

    #[test]
    fn ip_cidr() {
        let result = parse_hosts("1.1.1.1/30").unwrap();
        assert_eq!(result.len(), 4);
        let min = u32::from(Ipv4Addr::from_str("1.1.1.0").unwrap());
        let mut values = vec![];
        for i in 0..4 {
            values.push(Ipv4Addr::from(i + min));
        }
        assert_eq!(result, Arc::new(values));
    }

    #[test]
    fn ip_single() {
        assert_eq!(
            parse_hosts("192.168.1.1").unwrap(),
            Arc::new(vec![Ipv4Addr::from_str("192.168.1.1").unwrap()])
        );
    }
}
