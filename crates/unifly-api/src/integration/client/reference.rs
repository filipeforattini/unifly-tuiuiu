use uuid::Uuid;

use super::{Error, IntegrationClient, types};

impl IntegrationClient {
    // ── VPN (read-only) ──────────────────────────────────────────────

    pub async fn list_vpn_servers(
        &self,
        site_id: &Uuid,
        offset: i64,
        limit: i32,
    ) -> Result<types::Page<types::VpnServerResponse>, Error> {
        self.get_with_params(
            &format!("v1/sites/{site_id}/vpn/servers"),
            &[("offset", offset.to_string()), ("limit", limit.to_string())],
        )
        .await
    }

    pub async fn list_vpn_tunnels(
        &self,
        site_id: &Uuid,
        offset: i64,
        limit: i32,
    ) -> Result<types::Page<types::VpnTunnelResponse>, Error> {
        self.get_with_params(
            &format!("v1/sites/{site_id}/vpn/site-to-site-tunnels"),
            &[("offset", offset.to_string()), ("limit", limit.to_string())],
        )
        .await
    }

    // ── WAN (read-only) ──────────────────────────────────────────────

    pub async fn list_wans(
        &self,
        site_id: &Uuid,
        offset: i64,
        limit: i32,
    ) -> Result<types::Page<types::WanResponse>, Error> {
        self.get_with_params(
            &format!("v1/sites/{site_id}/wans"),
            &[("offset", offset.to_string()), ("limit", limit.to_string())],
        )
        .await
    }

    // ── DPI (read-only) ──────────────────────────────────────────────

    pub async fn list_dpi_categories(
        &self,
        _site_id: &Uuid,
        offset: i64,
        limit: i32,
    ) -> Result<types::Page<types::DpiCategoryResponse>, Error> {
        self.get_with_params(
            "v1/dpi/categories",
            &[("offset", offset.to_string()), ("limit", limit.to_string())],
        )
        .await
    }

    pub async fn list_dpi_applications(
        &self,
        _site_id: &Uuid,
        offset: i64,
        limit: i32,
    ) -> Result<types::Page<types::DpiApplicationResponse>, Error> {
        self.get_with_params(
            "v1/dpi/applications",
            &[("offset", offset.to_string()), ("limit", limit.to_string())],
        )
        .await
    }

    // ── RADIUS (read-only) ───────────────────────────────────────────

    pub async fn list_radius_profiles(
        &self,
        site_id: &Uuid,
        offset: i64,
        limit: i32,
    ) -> Result<types::Page<types::RadiusProfileResponse>, Error> {
        self.get_with_params(
            &format!("v1/sites/{site_id}/radius/profiles"),
            &[("offset", offset.to_string()), ("limit", limit.to_string())],
        )
        .await
    }

    // ── Countries (no site scope) ────────────────────────────────────

    pub async fn list_countries(
        &self,
        offset: i64,
        limit: i32,
    ) -> Result<types::Page<types::CountryResponse>, Error> {
        self.get_with_params(
            "v1/countries",
            &[("offset", offset.to_string()), ("limit", limit.to_string())],
        )
        .await
    }
}
