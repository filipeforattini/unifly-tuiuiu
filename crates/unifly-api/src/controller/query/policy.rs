use super::super::*;

impl Controller {
    pub async fn get_firewall_policy_ordering(
        &self,
        source_zone_id: &EntityId,
        destination_zone_id: &EntityId,
    ) -> Result<crate::integration_types::FirewallPolicyOrdering, CoreError> {
        let (client, site_id) =
            integration_site_context(self, "get_firewall_policy_ordering").await?;
        let source_zone_uuid = require_uuid(source_zone_id)?;
        let destination_zone_uuid = require_uuid(destination_zone_id)?;
        Ok(client
            .get_firewall_policy_ordering(&site_id, &source_zone_uuid, &destination_zone_uuid)
            .await?)
    }

    pub async fn get_acl_rule_ordering(
        &self,
    ) -> Result<crate::integration_types::AclRuleOrdering, CoreError> {
        let (client, site_id) = integration_site_context(self, "get_acl_rule_ordering").await?;
        Ok(client.get_acl_rule_ordering(&site_id).await?)
    }
}
