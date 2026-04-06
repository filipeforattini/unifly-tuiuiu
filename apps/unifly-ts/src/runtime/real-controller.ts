import type { ControllerConfig, ControllerSnapshot } from '../domain/types.js';
import { createInitialSnapshot, createSeedData } from '../services/demo-fixtures.js';
import { buildRealSnapshot } from '../services/unifi-normalize.js';
import { IntegrationApiClient } from '../transport/integration-client.js';
import { SessionApiClient } from '../transport/session-client.js';
import type { Controller } from './controller.js';
import { DataStore } from './store.js';

export class RealController implements Controller {
  readonly store: DataStore;
  readonly #integration: IntegrationApiClient;
  readonly #session: SessionApiClient;
  #refreshTimer: NodeJS.Timeout | null = null;
  #resolvedSessionSite: string;

  constructor(readonly config: ControllerConfig) {
    const apiKey = extractApiKey(config);
    this.#integration = new IntegrationApiClient(config.controllerUrl, apiKey);
    this.#session = new SessionApiClient(config.controllerUrl, apiKey);
    const initial = createInitialSnapshot(createSeedData());
    initial.runtime = {
      appMode: 'real',
      dataSource: 'fallback-demo',
      controllerUrl: config.controllerUrl,
      site: config.site,
      statusMessage: 'waiting for first live UniFi refresh',
      lastError: null,
    };
    this.store = new DataStore(initial);
    this.#resolvedSessionSite = config.site;
  }

  async connect(): Promise<void> {
    this.store.update((snapshot) => ({
      ...snapshot,
      connectionState: 'connecting',
      runtime: {
        ...snapshot.runtime,
        statusMessage: 'resolving site and fetching Integration + Session data',
        lastError: null,
      },
    }));
    await this.refresh();
    this.#refreshTimer = setInterval(() => {
      void this.refresh();
    }, this.config.refreshIntervalMs);
  }

  async disconnect(): Promise<void> {
    if (this.#refreshTimer) {
      clearInterval(this.#refreshTimer);
      this.#refreshTimer = null;
    }
    this.store.update((snapshot) => ({ ...snapshot, connectionState: 'disconnected' }));
  }

  async refresh(): Promise<void> {
    try {
      const sitePage = await this.#integration.listSites();
      const site = sitePage.data.find((candidate) =>
        [candidate.id, candidate.name, candidate.internalReference].includes(this.config.site),
      );

      if (!site) {
        throw new Error(`site '${this.config.site}' not found in Integration API`);
      }

      this.#resolvedSessionSite = site.internalReference;

      const [integrationDevices, integrationClients, integrationNetworks, sessionDevices, sessionClients, sessionEvents] =
        await Promise.all([
          this.#integration.listDevices(site.id),
          this.#integration.listClients(site.id),
          this.#integration.listNetworks(site.id),
          this.#session.listDevices(this.#resolvedSessionSite),
          this.#session.listClients(this.#resolvedSessionSite),
          this.#session.listEvents(this.#resolvedSessionSite, 20),
        ]);

      this.store.update((previous) =>
        buildRealSnapshot({
          integrationSite: site,
          integrationDevices: integrationDevices.data,
          integrationClients: integrationClients.data,
          integrationNetworks: integrationNetworks.data,
          sessionDevices,
          sessionClients,
          sessionEvents,
          previous,
        }),
      );
    } catch (error) {
      this.store.update((snapshot) => this.#markFailure(snapshot, error));
      throw error;
    }
  }

  toggleDemoPulse(): void {
    this.store.update((snapshot) => ({
      ...snapshot,
      demoPulseEnabled: !snapshot.demoPulseEnabled,
    }));
  }

  #markFailure(snapshot: ControllerSnapshot, error: unknown): ControllerSnapshot {
    const detail = error instanceof Error ? error.message : String(error);
    return {
      ...snapshot,
      connectionState: 'failed',
      lastEventAt: new Date().toISOString(),
      runtime: {
        ...snapshot.runtime,
        dataSource: 'fallback-demo',
        statusMessage: 'live UniFi refresh failed; showing last known snapshot',
        lastError: detail,
      },
      events: [
        {
          id: { kind: 'legacy' as const, value: `bootstrap-${Date.now()}` },
          timestamp: new Date().toISOString(),
          category: 'system' as const,
          severity: 'error' as const,
          title: 'real controller refresh failed',
          detail,
        },
        ...snapshot.events,
      ].slice(0, 20),
    };
  }
}

function extractApiKey(config: ControllerConfig): string {
  if (config.auth.kind === 'apiKey') {
    return config.auth.apiKey;
  }

  if (config.auth.kind === 'hybrid') {
    return config.auth.apiKey;
  }

  throw new Error('real controller currently requires UNIFI_API_KEY');
}
