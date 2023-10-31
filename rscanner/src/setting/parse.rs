use std::net::Ipv4Addr;
use std::str::FromStr;
use std::sync::Arc;

use ipnetwork::Ipv4Network;

use crate::err::APPError;

/// parse input like
/// single ip `192.168.1.1`
/// ip with cidr `192.168.1.0/12`
/// combines all formats with comma separate `192.168.1.1,192.168.1.0,192.168.2.1/24`
pub fn parse_hosts(value: &str) -> anyhow::Result<Arc<Vec<Ipv4Addr>>> {
    let splits = value.split(',');
    let mut full_values = vec![];
    for i in splits {
        let results = parse_ipv4_cidr(i)?;
        full_values.extend(results);
    }
    Ok(Arc::new(full_values))
}

pub fn parse_ipv4_cidr(value: &str) -> anyhow::Result<Vec<Ipv4Addr>> {
    Ok(Ipv4Network::from_str(value)?
        .into_iter()
        .collect::<Vec<Ipv4Addr>>())
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

pub fn parse_ports_range(input: &str) -> anyhow::Result<Vec<u16>> {
    let range = input
        .split('-')
        .map(str::parse)
        .collect::<Result<Vec<u16>, std::num::ParseIntError>>()?;
    if let [start, end] = range.as_slice() {
        let result = (*start..*end).collect::<Vec<u16>>();
        return Ok(result);
    }
    Err(APPError::PortFormatError.into())
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
        let result = parse_ipv4_cidr("1.1.1.1/30").unwrap();
        assert_eq!(result.len(), 4);
        let min = u32::from(Ipv4Addr::from_str("1.1.1.0").unwrap());
        let mut values = vec![];
        for i in 0..4 {
            values.push(Ipv4Addr::from(i + min));
        }
        assert_eq!(result, values);
    }

    #[test]
    fn ip_single() {
        assert_eq!(
            parse_ipv4_cidr("192.168.1.1").unwrap(),
            vec![Ipv4Addr::from_str("192.168.1.1").unwrap()]
        );
    }
}
