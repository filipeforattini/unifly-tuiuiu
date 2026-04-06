import type {
  ClientSummary,
  ControllerConfig,
  ControllerSnapshot,
  DeviceSummary,
  EventSummary,
  NetworkSummary,
  ThroughputPoint,
} from '../domain/types.js';
import { createInitialSnapshot, createSeedData } from '../services/demo-fixtures.js';
import type { Controller } from './controller.js';
import { DataStore } from './store.js';

const EVENT_TICK_MS = 2_500;

export class DemoController implements Controller {
  readonly store: DataStore;
  readonly #seed = createSeedData();
  #refreshTimer: NodeJS.Timeout | null = null;
  #eventTimer: NodeJS.Timeout | null = null;
  #tick = 0;

  constructor(readonly config: ControllerConfig) {
    const initial = createInitialSnapshot(this.#seed);
    initial.runtime = {
      appMode: 'demo',
      dataSource: 'demo',
      controllerUrl: config.controllerUrl,
      site: config.site,
      statusMessage: 'demo mode using synthetic UniFi-like data',
      lastError: null,
    };
    this.store = new DataStore(initial);
  }

  async connect(): Promise<void> {
    this.store.update((snapshot) => ({ ...snapshot, connectionState: 'connecting' }));

    await new Promise((resolve) => setTimeout(resolve, 150));

    this.store.update((snapshot) => ({
      ...snapshot,
      connectionState: 'connected',
      lastRefreshAt: new Date().toISOString(),
      runtime: {
        ...snapshot.runtime,
        statusMessage: 'demo stream connected',
        lastError: null,
      },
    }));

    this.#startTimers();
  }

  async disconnect(): Promise<void> {
    this.#stopTimers();
    this.store.update((snapshot) => ({ ...snapshot, connectionState: 'disconnected' }));
  }

  async refresh(): Promise<void> {
    this.#tick += 1;
    this.store.update((snapshot) => this.#nextSnapshot(snapshot, true));
  }

  toggleDemoPulse(): void {
    this.store.update((snapshot) => ({
      ...snapshot,
      demoPulseEnabled: !snapshot.demoPulseEnabled,
    }));
  }

  #startTimers(): void {
    this.#stopTimers();

    this.#refreshTimer = setInterval(() => {
      this.#tick += 1;
      this.store.update((snapshot) => this.#nextSnapshot(snapshot, false));
    }, this.config.refreshIntervalMs);

    this.#eventTimer = setInterval(() => {
      this.store.update((snapshot) => this.#appendEvent(snapshot));
    }, EVENT_TICK_MS);
  }

  #stopTimers(): void {
    if (this.#refreshTimer) {
      clearInterval(this.#refreshTimer);
      this.#refreshTimer = null;
    }

    if (this.#eventTimer) {
      clearInterval(this.#eventTimer);
      this.#eventTimer = null;
    }
  }

  #nextSnapshot(snapshot: ControllerSnapshot, forceSpike: boolean): ControllerSnapshot {
    const amplitude = snapshot.demoPulseEnabled ? 1.35 : 0.55;
    const wave = Math.sin(this.#tick / 1.7) * amplitude;
    const pulse = forceSpike ? 1.12 : 1;
    const now = new Date().toISOString();
    const devices = this.#seed.devices.map((device, index) =>
      this.#evolveDevice(device, wave, pulse, index),
    );
    const clients = this.#seed.clients.map((client, index) =>
      this.#evolveClient(client, wave, pulse, index),
    );
    const networks = this.#seed.networks.map((network, index) =>
      this.#evolveNetwork(network, wave, pulse, index),
    );
    const totalTxMbps = devices.reduce((sum, device) => sum + device.txMbps, 0);
    const totalRxMbps = devices.reduce((sum, device) => sum + device.rxMbps, 0);
    const activeClients = clients.length;
    const onlineDevices = devices.filter((device) => device.status === 'online').length;
    const throughputPoint: ThroughputPoint = {
      timestamp: now,
      txMbps: round1(totalTxMbps),
      rxMbps: round1(totalRxMbps),
    };

    return {
      ...snapshot,
      lastRefreshAt: now,
      devices,
      clients,
      networks,
      metrics: {
        activeClients,
        onlineDevices,
        networks: networks.length,
        eventsLastHour: snapshot.events.length + 12,
        totalTxMbps: round1(totalTxMbps),
        totalRxMbps: round1(totalRxMbps),
        siteHealth: {
          gatewayLatencyMs: round1(8 + this.#clamp(10 + wave * 4, 0, 50)),
          wanUptimePct: round1(this.#clamp(99.98 - Math.abs(wave) * 0.03, 97, 100)),
          wifiExperiencePct: round1(this.#clamp(95 + wave * 2.5, 70, 100)),
          packetLossPct: round1(this.#clamp(0.12 + Math.abs(wave) * 0.09, 0, 3)),
        },
        throughputHistory: [...snapshot.metrics.throughputHistory.slice(-35), throughputPoint],
      },
    };
  }

  #appendEvent(snapshot: ControllerSnapshot): ControllerSnapshot {
    const eventIndex = this.#tick % this.#seed.eventTemplates.length;
    const template = this.#seed.eventTemplates[eventIndex];
    const now = new Date().toISOString();
    const event: EventSummary = {
      ...template,
      id: { kind: 'legacy', value: `evt-${Date.now()}` },
      timestamp: now,
    };

    return {
      ...snapshot,
      lastEventAt: now,
      events: [event, ...snapshot.events].slice(0, 18),
    };
  }

  #evolveDevice(device: DeviceSummary, wave: number, pulse: number, index: number): DeviceSummary {
    const drift = Math.sin(this.#tick / (2.4 + index)) * 0.8;
    return {
      ...device,
      cpuPct: round1(this.#clamp(device.cpuPct + wave * 8 + drift * 5, 4, 96)),
      memPct: round1(this.#clamp(device.memPct + wave * 4 + drift * 2, 10, 92)),
      txMbps: round1(this.#clamp(device.txMbps * pulse + wave * 18 + index * 2, 0.1, 950)),
      rxMbps: round1(this.#clamp(device.rxMbps * pulse + wave * 12 + index, 0.1, 750)),
      clients: Math.round(this.#clamp(device.clients + wave * 4 + drift * 3, 0, 240)),
      status: this.#deviceStatus(device.status, wave, index),
    };
  }

  #evolveClient(client: ClientSummary, wave: number, pulse: number, index: number): ClientSummary {
    const drift = Math.cos(this.#tick / (1.9 + index)) * 1.2;
    return {
      ...client,
      signalDbm: Math.round(this.#clamp(client.signalDbm + wave * 4 + drift, -85, -42)),
      experiencePct: round1(this.#clamp(client.experiencePct + wave * 3 + drift, 62, 100)),
      trafficMbps: round1(this.#clamp(client.trafficMbps * pulse + wave * 5, 0.1, 180)),
      roaming: index % 5 === this.#tick % 5,
    };
  }

  #evolveNetwork(network: NetworkSummary, wave: number, pulse: number, index: number): NetworkSummary {
    return {
      ...network,
      clients: Math.round(this.#clamp(network.clients * pulse + wave * 10 + index * 2, 2, 300)),
      healthPct: round1(this.#clamp(network.healthPct + wave * 2.5, 70, 100)),
    };
  }

  #deviceStatus(
    initialStatus: DeviceSummary['status'],
    wave: number,
    index: number,
  ): DeviceSummary['status'] {
    if (initialStatus === 'offline') {
      return index % 2 === 0 && wave > 0.6 ? 'degraded' : 'offline';
    }

    if (wave > 0.95 && index === 1) {
      return 'degraded';
    }

    return 'online';
  }

  #clamp(value: number, min: number, max: number): number {
    return Math.min(max, Math.max(min, value));
  }
}

function round1(value: number): number {
  return Math.round(value * 10) / 10;
}
