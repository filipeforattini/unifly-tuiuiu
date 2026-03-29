use uuid::Uuid;

use super::{Error, IntegrationClient, types};

impl IntegrationClient {
    // ── ACL Rules ────────────────────────────────────────────────────

    pub async fn list_acl_rules(
        &self,
        site_id: &Uuid,
        offset: i64,
        limit: i32,
    ) -> Result<types::Page<types::AclRuleResponse>, Error> {
        self.get_with_params(
            &format!("v1/sites/{site_id}/acl-rules"),
            &[("offset", offset.to_string()), ("limit", limit.to_string())],
        )
        .await
    }

    pub async fn get_acl_rule(
        &self,
        site_id: &Uuid,
        rule_id: &Uuid,
    ) -> Result<types::AclRuleResponse, Error> {
        self.get(&format!("v1/sites/{site_id}/acl-rules/{rule_id}"))
            .await
    }

    pub async fn create_acl_rule(
        &self,
        site_id: &Uuid,
        body: &types::AclRuleCreateUpdate,
    ) -> Result<types::AclRuleResponse, Error> {
        self.post(&format!("v1/sites/{site_id}/acl-rules"), body)
            .await
    }

    pub async fn update_acl_rule(
        &self,
        site_id: &Uuid,
        rule_id: &Uuid,
        body: &types::AclRuleCreateUpdate,
    ) -> Result<types::AclRuleResponse, Error> {
        self.put(&format!("v1/sites/{site_id}/acl-rules/{rule_id}"), body)
            .await
    }

    pub async fn delete_acl_rule(&self, site_id: &Uuid, rule_id: &Uuid) -> Result<(), Error> {
        self.delete(&format!("v1/sites/{site_id}/acl-rules/{rule_id}"))
            .await
    }

    pub async fn get_acl_rule_ordering(
        &self,
        site_id: &Uuid,
    ) -> Result<types::AclRuleOrdering, Error> {
        self.get(&format!("v1/sites/{site_id}/acl-rules/ordering"))
            .await
    }

    pub async fn set_acl_rule_ordering(
        &self,
        site_id: &Uuid,
        body: &types::AclRuleOrdering,
    ) -> Result<types::AclRuleOrdering, Error> {
        self.put(&format!("v1/sites/{site_id}/acl-rules/ordering"), body)
            .await
    }

    // ── DNS Policies ─────────────────────────────────────────────────

    pub async fn list_dns_policies(
        &self,
        site_id: &Uuid,
        offset: i64,
        limit: i32,
    ) -> Result<types::Page<types::DnsPolicyResponse>, Error> {
        self.get_with_params(
            &format!("v1/sites/{site_id}/dns/policies"),
            &[("offset", offset.to_string()), ("limit", limit.to_string())],
        )
        .await
    }

    pub async fn get_dns_policy(
        &self,
        site_id: &Uuid,
        dns_id: &Uuid,
    ) -> Result<types::DnsPolicyResponse, Error> {
        self.get(&format!("v1/sites/{site_id}/dns/policies/{dns_id}"))
            .await
    }

    pub async fn create_dns_policy(
        &self,
        site_id: &Uuid,
        body: &types::DnsPolicyCreateUpdate,
    ) -> Result<types::DnsPolicyResponse, Error> {
        self.post(&format!("v1/sites/{site_id}/dns/policies"), body)
            .await
    }

    pub async fn update_dns_policy(
        &self,
        site_id: &Uuid,
        dns_id: &Uuid,
        body: &types::DnsPolicyCreateUpdate,
    ) -> Result<types::DnsPolicyResponse, Error> {
        self.put(&format!("v1/sites/{site_id}/dns/policies/{dns_id}"), body)
            .await
    }

    pub async fn delete_dns_policy(&self, site_id: &Uuid, dns_id: &Uuid) -> Result<(), Error> {
        self.delete(&format!("v1/sites/{site_id}/dns/policies/{dns_id}"))
            .await
    }

    // ── Traffic Matching Lists ───────────────────────────────────────

    pub async fn list_traffic_matching_lists(
        &self,
        site_id: &Uuid,
        offset: i64,
        limit: i32,
    ) -> Result<types::Page<types::TrafficMatchingListResponse>, Error> {
        self.get_with_params(
            &format!("v1/sites/{site_id}/traffic-matching-lists"),
            &[("offset", offset.to_string()), ("limit", limit.to_string())],
        )
        .await
    }

    pub async fn get_traffic_matching_list(
        &self,
        site_id: &Uuid,
        list_id: &Uuid,
    ) -> Result<types::TrafficMatchingListResponse, Error> {
        self.get(&format!(
            "v1/sites/{site_id}/traffic-matching-lists/{list_id}"
        ))
        .await
    }

    pub async fn create_traffic_matching_list(
        &self,
        site_id: &Uuid,
        body: &types::TrafficMatchingListCreateUpdate,
    ) -> Result<types::TrafficMatchingListResponse, Error> {
        self.post(&format!("v1/sites/{site_id}/traffic-matching-lists"), body)
            .await
    }

    pub async fn update_traffic_matching_list(
        &self,
        site_id: &Uuid,
        list_id: &Uuid,
        body: &types::TrafficMatchingListCreateUpdate,
    ) -> Result<types::TrafficMatchingListResponse, Error> {
        self.put(
            &format!("v1/sites/{site_id}/traffic-matching-lists/{list_id}"),
            body,
        )
        .await
    }

    pub async fn delete_traffic_matching_list(
        &self,
        site_id: &Uuid,
        list_id: &Uuid,
    ) -> Result<(), Error> {
        self.delete(&format!(
            "v1/sites/{site_id}/traffic-matching-lists/{list_id}"
        ))
        .await
    }

    // ── Hotspot Vouchers ─────────────────────────────────────────────

    pub async fn list_vouchers(
        &self,
        site_id: &Uuid,
        offset: i64,
        limit: i32,
    ) -> Result<types::Page<types::VoucherResponse>, Error> {
        self.get_with_params(
            &format!("v1/sites/{site_id}/hotspot/vouchers"),
            &[("offset", offset.to_string()), ("limit", limit.to_string())],
        )
        .await
    }

    pub async fn get_voucher(
        &self,
        site_id: &Uuid,
        voucher_id: &Uuid,
    ) -> Result<types::VoucherResponse, Error> {
        self.get(&format!("v1/sites/{site_id}/hotspot/vouchers/{voucher_id}"))
            .await
    }

    pub async fn create_vouchers(
        &self,
        site_id: &Uuid,
        body: &types::VoucherCreateRequest,
    ) -> Result<Vec<types::VoucherResponse>, Error> {
        self.post(&format!("v1/sites/{site_id}/hotspot/vouchers"), body)
            .await
    }

    pub async fn delete_voucher(
        &self,
        site_id: &Uuid,
        voucher_id: &Uuid,
    ) -> Result<types::VoucherDeletionResults, Error> {
        self.delete_with_response(&format!("v1/sites/{site_id}/hotspot/vouchers/{voucher_id}"))
            .await
    }

    pub async fn purge_vouchers(
        &self,
        site_id: &Uuid,
        filter: &str,
    ) -> Result<types::VoucherDeletionResults, Error> {
        self.delete_with_params(
            &format!("v1/sites/{site_id}/hotspot/vouchers"),
            &[("filter", filter.to_owned())],
        )
        .await
    }
}
