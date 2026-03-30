use unifly_api::{CreateAclRuleRequest, EntityId, UpdateAclRuleRequest, model::FirewallAction};

use crate::cli::args::{AclAction, AclRuleType};
use crate::cli::error::CliError;

fn acl_rule_type_name(rule_type: &AclRuleType) -> &'static str {
    match rule_type {
        AclRuleType::Ipv4 => "IP",
        AclRuleType::Mac => "MAC",
    }
}

fn map_acl_action(action: &AclAction) -> FirewallAction {
    match action {
        AclAction::Allow => FirewallAction::Allow,
        AclAction::Block => FirewallAction::Block,
    }
}

#[allow(clippy::too_many_arguments)]
pub(super) fn build_acl_create_request(
    name: Option<String>,
    rule_type: Option<AclRuleType>,
    action: Option<AclAction>,
    source_zone: Option<String>,
    dest_zone: Option<String>,
    protocol: Option<String>,
    source_port: Option<String>,
    destination_port: Option<String>,
) -> Result<CreateAclRuleRequest, CliError> {
    let name = name.ok_or_else(|| CliError::Validation {
        field: "name".into(),
        reason: "ACL create requires --name".into(),
    })?;
    let rule_type = rule_type.ok_or_else(|| CliError::Validation {
        field: "rule_type".into(),
        reason: "ACL create requires --rule-type".into(),
    })?;
    let action = action.ok_or_else(|| CliError::Validation {
        field: "action".into(),
        reason: "ACL create requires --action".into(),
    })?;
    let source_zone_id = EntityId::from(source_zone.ok_or_else(|| CliError::Validation {
        field: "source_zone".into(),
        reason: "ACL create requires --source-zone".into(),
    })?);
    let destination_zone_id = EntityId::from(dest_zone.ok_or_else(|| CliError::Validation {
        field: "dest_zone".into(),
        reason: "ACL create requires --dest-zone".into(),
    })?);

    Ok(CreateAclRuleRequest {
        name,
        rule_type: acl_rule_type_name(&rule_type).into(),
        action: map_acl_action(&action),
        source_zone_id,
        destination_zone_id,
        description: None,
        protocol,
        source_port,
        destination_port,
        source_filter: None,
        destination_filter: None,
        enforcing_device_filter: None,
        enabled: true,
    })
}

#[allow(clippy::too_many_arguments)]
pub(super) fn build_acl_update_request(
    name: Option<String>,
    rule_type: Option<AclRuleType>,
    action: Option<&AclAction>,
    source_zone: Option<String>,
    dest_zone: Option<String>,
    protocol: Option<String>,
    source_port: Option<String>,
    destination_port: Option<String>,
    enabled: Option<bool>,
) -> Result<UpdateAclRuleRequest, CliError> {
    let update = UpdateAclRuleRequest {
        name,
        rule_type: rule_type.map(|kind| acl_rule_type_name(&kind).to_owned()),
        action: action.map(map_acl_action),
        enabled,
        description: None,
        source_zone_id: source_zone.map(EntityId::from),
        destination_zone_id: dest_zone.map(EntityId::from),
        protocol,
        source_port,
        destination_port,
        source_filter: None,
        destination_filter: None,
        enforcing_device_filter: None,
    };

    let has_changes = update.name.is_some()
        || update.rule_type.is_some()
        || update.action.is_some()
        || update.enabled.is_some()
        || update.source_zone_id.is_some()
        || update.destination_zone_id.is_some()
        || update.protocol.is_some()
        || update.source_port.is_some()
        || update.destination_port.is_some();

    if has_changes {
        Ok(update)
    } else {
        Err(CliError::Validation {
            field: "update".into(),
            reason: "ACL update requires at least one change or --from-file".into(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::{build_acl_create_request, build_acl_update_request};

    #[test]
    fn build_acl_create_request_uses_api_rule_type_names() {
        let create = build_acl_create_request(
            Some("Block cameras".into()),
            Some(crate::cli::args::AclRuleType::Mac),
            Some(crate::cli::args::AclAction::Block),
            Some("src-zone".into()),
            Some("dst-zone".into()),
            Some("TCP".into()),
            Some("1234".into()),
            Some("443".into()),
        )
        .expect("create request should build");

        assert_eq!(create.rule_type, "MAC");
        assert_eq!(create.source_zone_id.to_string(), "src-zone");
        assert_eq!(create.destination_zone_id.to_string(), "dst-zone");
    }

    #[test]
    fn build_acl_update_request_accepts_inline_fields() {
        let update = build_acl_update_request(
            Some("Updated".into()),
            Some(crate::cli::args::AclRuleType::Ipv4),
            Some(&crate::cli::args::AclAction::Allow),
            Some("src-zone".into()),
            Some("dst-zone".into()),
            Some("UDP".into()),
            Some("53".into()),
            Some("5353".into()),
            Some(false),
        )
        .expect("update request should build");

        assert_eq!(update.rule_type.as_deref(), Some("IP"));
        assert_eq!(update.protocol.as_deref(), Some("UDP"));
        assert_eq!(
            update
                .source_zone_id
                .as_ref()
                .map(ToString::to_string)
                .as_deref(),
            Some("src-zone")
        );
        assert_eq!(
            update
                .destination_zone_id
                .as_ref()
                .map(ToString::to_string)
                .as_deref(),
            Some("dst-zone")
        );
        assert_eq!(update.enabled, Some(false));
    }

    #[test]
    fn build_acl_update_request_rejects_empty_inline_update() {
        let err = build_acl_update_request(None, None, None, None, None, None, None, None, None)
            .expect_err("empty update should fail");
        match err {
            crate::cli::error::CliError::Validation { field, .. } => assert_eq!(field, "update"),
            other => panic!("expected validation error, got {other:?}"),
        }
    }
}
