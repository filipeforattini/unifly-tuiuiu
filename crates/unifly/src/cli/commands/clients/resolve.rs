use std::net::Ipv4Addr;

use unifly_api::{Controller, EntityId};

use crate::cli::error::CliError;

pub(super) fn resolve_network(
    controller: &Controller,
    name_or_id: Option<&str>,
    ip: Ipv4Addr,
) -> Result<EntityId, CliError> {
    let networks = controller.networks_snapshot();

    if let Some(needle) = name_or_id {
        return networks
            .iter()
            .find(|network| {
                network.name.eq_ignore_ascii_case(needle) || network.id.to_string() == needle
            })
            .map(|network| network.id.clone())
            .ok_or_else(|| CliError::NotFound {
                resource_type: "network".into(),
                identifier: needle.into(),
                list_command: "networks list".into(),
            });
    }

    networks
        .iter()
        .find(|network| {
            network
                .subnet
                .as_deref()
                .and_then(parse_cidr)
                .is_some_and(|(network_addr, prefix)| subnet_contains(ip, network_addr, prefix))
        })
        .map(|network| network.id.clone())
        .ok_or_else(|| CliError::Validation {
            field: "network".into(),
            reason: format!(
                "could not auto-detect network for IP {ip}; use --network to specify explicitly"
            ),
        })
}

fn subnet_contains(ip: Ipv4Addr, network_addr: Ipv4Addr, prefix: u32) -> bool {
    let mask = if prefix == 0 {
        0
    } else {
        u32::MAX << (32 - prefix)
    };
    (u32::from(ip) & mask) == (u32::from(network_addr) & mask)
}

fn parse_cidr(value: &str) -> Option<(Ipv4Addr, u32)> {
    let (addr_str, prefix_str) = value.split_once('/')?;
    let addr: Ipv4Addr = addr_str.parse().ok()?;
    let prefix: u32 = prefix_str.parse().ok()?;
    Some((addr, prefix))
}

#[cfg(test)]
mod tests {
    use std::net::Ipv4Addr;

    use super::{parse_cidr, subnet_contains};

    #[test]
    fn parse_cidr_extracts_network_and_prefix() {
        assert_eq!(
            parse_cidr("10.4.22.1/24"),
            Some((Ipv4Addr::new(10, 4, 22, 1), 24))
        );
    }

    #[test]
    fn parse_cidr_rejects_invalid_values() {
        assert_eq!(parse_cidr("10.4.22.1"), None);
        assert_eq!(parse_cidr("nope/24"), None);
    }

    #[test]
    fn subnet_contains_matches_ip_inside_network() {
        assert!(subnet_contains(
            Ipv4Addr::new(10, 4, 22, 88),
            Ipv4Addr::new(10, 4, 22, 1),
            24,
        ));
    }

    #[test]
    fn subnet_contains_rejects_ip_outside_network() {
        assert!(!subnet_contains(
            Ipv4Addr::new(10, 4, 23, 88),
            Ipv4Addr::new(10, 4, 22, 1),
            24,
        ));
    }
}
