use std::sync::Arc;

use tokio::sync::{broadcast, watch};

use crate::model::{
    AclRule, Client, Device, DnsPolicy, Event, FirewallPolicy, FirewallZone, HealthSummary,
    NatPolicy, Network, Site, TrafficMatchingList, Voucher, WifiBroadcast,
};
use crate::stream::EntityStream;

use super::{ConnectionState, Controller};

impl Controller {
    // ── State observation ────────────────────────────────────────

    /// Subscribe to connection state changes.
    pub fn connection_state(&self) -> watch::Receiver<ConnectionState> {
        self.inner.connection_state.subscribe()
    }

    /// Subscribe to the event broadcast stream.
    pub fn events(&self) -> broadcast::Receiver<Arc<Event>> {
        self.inner.event_tx.subscribe()
    }

    // ── Snapshot accessors (delegate to DataStore) ───────────────

    pub fn devices_snapshot(&self) -> Arc<Vec<Arc<Device>>> {
        self.inner.store.devices_snapshot()
    }

    pub fn clients_snapshot(&self) -> Arc<Vec<Arc<Client>>> {
        self.inner.store.clients_snapshot()
    }

    pub fn networks_snapshot(&self) -> Arc<Vec<Arc<Network>>> {
        self.inner.store.networks_snapshot()
    }

    pub fn wifi_broadcasts_snapshot(&self) -> Arc<Vec<Arc<WifiBroadcast>>> {
        self.inner.store.wifi_broadcasts_snapshot()
    }

    pub fn firewall_policies_snapshot(&self) -> Arc<Vec<Arc<FirewallPolicy>>> {
        self.inner.store.firewall_policies_snapshot()
    }

    pub fn firewall_zones_snapshot(&self) -> Arc<Vec<Arc<FirewallZone>>> {
        self.inner.store.firewall_zones_snapshot()
    }

    pub fn acl_rules_snapshot(&self) -> Arc<Vec<Arc<AclRule>>> {
        self.inner.store.acl_rules_snapshot()
    }

    pub fn nat_policies_snapshot(&self) -> Arc<Vec<Arc<NatPolicy>>> {
        self.inner.store.nat_policies_snapshot()
    }

    pub fn dns_policies_snapshot(&self) -> Arc<Vec<Arc<DnsPolicy>>> {
        self.inner.store.dns_policies_snapshot()
    }

    pub fn vouchers_snapshot(&self) -> Arc<Vec<Arc<Voucher>>> {
        self.inner.store.vouchers_snapshot()
    }

    pub fn sites_snapshot(&self) -> Arc<Vec<Arc<Site>>> {
        self.inner.store.sites_snapshot()
    }

    pub fn events_snapshot(&self) -> Arc<Vec<Arc<Event>>> {
        self.inner.store.events_snapshot()
    }

    pub fn traffic_matching_lists_snapshot(&self) -> Arc<Vec<Arc<TrafficMatchingList>>> {
        self.inner.store.traffic_matching_lists_snapshot()
    }

    // ── Stream accessors (delegate to DataStore) ─────────────────

    pub fn devices(&self) -> EntityStream<Device> {
        self.inner.store.subscribe_devices()
    }

    pub fn clients(&self) -> EntityStream<Client> {
        self.inner.store.subscribe_clients()
    }

    pub fn networks(&self) -> EntityStream<Network> {
        self.inner.store.subscribe_networks()
    }

    pub fn wifi_broadcasts(&self) -> EntityStream<WifiBroadcast> {
        self.inner.store.subscribe_wifi_broadcasts()
    }

    pub fn firewall_policies(&self) -> EntityStream<FirewallPolicy> {
        self.inner.store.subscribe_firewall_policies()
    }

    pub fn firewall_zones(&self) -> EntityStream<FirewallZone> {
        self.inner.store.subscribe_firewall_zones()
    }

    pub fn acl_rules(&self) -> EntityStream<AclRule> {
        self.inner.store.subscribe_acl_rules()
    }

    pub fn nat_policies(&self) -> EntityStream<NatPolicy> {
        self.inner.store.subscribe_nat_policies()
    }

    pub fn dns_policies(&self) -> EntityStream<DnsPolicy> {
        self.inner.store.subscribe_dns_policies()
    }

    pub fn vouchers(&self) -> EntityStream<Voucher> {
        self.inner.store.subscribe_vouchers()
    }

    pub fn sites(&self) -> EntityStream<Site> {
        self.inner.store.subscribe_sites()
    }

    pub fn traffic_matching_lists(&self) -> EntityStream<TrafficMatchingList> {
        self.inner.store.subscribe_traffic_matching_lists()
    }

    /// Subscribe to site health updates (WAN IP, latency, bandwidth rates).
    pub fn site_health(&self) -> watch::Receiver<Arc<Vec<HealthSummary>>> {
        self.inner.store.subscribe_site_health()
    }

    /// Drain warnings accumulated during connect (e.g. Legacy auth failure).
    pub async fn take_warnings(&self) -> Vec<String> {
        std::mem::take(&mut *self.inner.warnings.lock().await)
    }

    /// Whether a logged-in Legacy client is available for legacy-only features.
    pub async fn has_legacy_access(&self) -> bool {
        self.inner.legacy_client.lock().await.is_some()
    }

    /// Whether the Integration API is available for integration-backed features.
    pub async fn has_integration_access(&self) -> bool {
        self.inner.integration_client.lock().await.is_some()
            && self.inner.site_id.lock().await.is_some()
    }
}
