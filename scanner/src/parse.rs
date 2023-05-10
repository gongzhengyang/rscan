use ipnetwork::Ipv4Network;
use std::str::FromStr;

use crate::err::APPError;

/// parse input like
/// single ip `192.168.1.1`
/// ip with cidr `192.168.1.0/12`
/// combines all formats with comma separate `192.168.1.1,192.168.1.0,192.168.2.1/24`
pub fn parse_ipv4_values(value: &str) -> anyhow::Result<Vec<u32>> {
    let splits = value.split(',');
    let mut full_values = vec![];
    for i in splits {
        full_values.extend(parse_ipv4_cidr(i)?);
    }
    Ok(full_values)
}

pub fn parse_ipv4_cidr(value: &str) -> anyhow::Result<Vec<u32>> {
    Ok(Ipv4Network::from_str(value)?
        .iter()
        .map(|x| u32::from(x))
        .collect::<Vec<u32>>())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::Ipv4Addr;

    #[test]
    fn multi_ipv4_values() {
        let results = parse_ipv4_values("192.168.1.1,192.168.1.2,1.1.1.1/24").unwrap();
        let mut compare = vec![];
        let first = u32::from(Ipv4Addr::from_str("192.168.1.1").unwrap());
        compare.push(first);
        compare.push(first + 1);
        let cidr_start = u32::from(Ipv4Addr::from_str("1.1.1.0").unwrap());
        compare.extend(
            (0..256)
                .into_iter()
                .map(|x| x + cidr_start)
                .collect::<Vec<u32>>(),
        );
        assert_eq!(results, compare);
    }

    #[test]
    fn ip_cidr() {
        let result = parse_ipv4_cidr("1.1.1.1/30").unwrap();
        assert_eq!(result.len(), 4);
        let min = u32::from(Ipv4Addr::from_str("1.1.1.0").unwrap());
        let mut values = vec![];
        for i in 0..4 {
            values.push(i + min);
        }
        assert_eq!(result, values);
    }

    #[test]
    fn ip_single() {
        assert_eq!(
            parse_ipv4_cidr("192.168.1.1").unwrap(),
            vec![u32::from(Ipv4Addr::from_str("192.168.1.1").unwrap())]
        );
    }
}
