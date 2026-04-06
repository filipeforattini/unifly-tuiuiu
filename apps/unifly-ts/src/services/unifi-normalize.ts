import { entityId, normalizeMac } from '../domain/identity.js';
import type {
  ClientSummary,
  ControllerSnapshot,
  DeviceSummary,
  EventSummary,
  NetworkSummary,
} from '../domain/types.js';
import type {
  IntegrationClientEntity,
  IntegrationDevice,
  IntegrationNetwork,
  IntegrationSite,
} from '../transport/integration-client.js';
import type { SessionClientEntry, SessionDevice, SessionEvent } from '../transport/session-client.js';

export interface RealSnapshotInput {
  integrationSite: IntegrationSite;
  integrationDevices: IntegrationDevice[];
  integrationClients: IntegrationClientEntity[];
  integrationNetworks: IntegrationNetwork[];
  sessionDevices: SessionDevice[];
  sessionClients: SessionClientEntry[];
  sessionEvents: SessionEvent[];
  previous: ControllerSnapshot;
}

export function buildRealSnapshot(input: RealSnapshotInput): ControllerSnapshot {
  const devices = mergeDevices(input.integrationDevices, input.sessionDevices);
  const clients = mergeClients(input.integrationClients, input.sessionClients, devices);
  const networks = mapNetworks(input.integrationNetworks, clients);
  const events = mapEvents(input.sessionEvents);
  const totalTxMbps = devices.reduce((sum, device) => sum + device.txMbps, 0);
  const totalRxMbps = devices.reduce((sum, device) => sum + device.rxMbps, 0);
  const now = new Date().toISOString();

  return {
    ...input.previous,
    connectionState: 'connected',
    lastRefreshAt: now,
    lastEventAt: events[0]?.timestamp ?? input.previous.lastEventAt,
    devices,
    clients,
    networks,
    events,
    metrics: {
      activeClients: clients.length,
      onlineDevices: devices.filter((device) => device.status === 'online').length,
      networks: networks.length,
      eventsLastHour: events.length,
      totalTxMbps: round1(totalTxMbps),
      totalRxMbps: round1(totalRxMbps),
      siteHealth: {
        gatewayLatencyMs: inferGatewayLatency(events),
        wanUptimePct: 99.9,
        wifiExperiencePct: round1(
          clients.length === 0
            ? 100
            : clients.reduce((sum, client) => sum + client.experiencePct, 0) / clients.length,
        ),
        packetLossPct: inferPacketLoss(events),
      },
      throughputHistory: [
        ...input.previous.metrics.throughputHistory.slice(-35),
        { timestamp: now, txMbps: round1(totalTxMbps), rxMbps: round1(totalRxMbps) },
      ],
    },
  };
}

function mergeDevices(
  integrationDevices: IntegrationDevice[],
  sessionDevices: SessionDevice[],
): DeviceSummary[] {
  const sessionByMac = new Map(sessionDevices.map((device) => [normalizeMac(device.mac), device]));

  return integrationDevices.map((device) => {
    const session = sessionByMac.get(normalizeMac(device.macAddress));
    const txMbps = bytesToMbps(session?.tx_bytes);
    const rxMbps = bytesToMbps(session?.rx_bytes);
    const memPct =
      session?.sys_stats?.mem_total && session.sys_stats.mem_used
        ? (session.sys_stats.mem_used / session.sys_stats.mem_total) * 100
        : 0;

    return {
      id: entityId(device.id),
      mac: normalizeMac(device.macAddress),
      name: device.name,
      model: device.model,
      ip: device.ipAddress ?? session?.ip ?? '-',
      status: mapDeviceStatus(device.state, session?.state),
      cpuPct: round1(Number(session?.sys_stats?.cpu ?? '0')),
      memPct: round1(memPct),
      clients: session?.num_sta ?? 0,
      uplink: session?.uplink ?? 'n/a',
      txMbps,
      rxMbps,
    };
  });
}

function mergeClients(
  integrationClients: IntegrationClientEntity[],
  sessionClients: SessionClientEntry[],
  devices: DeviceSummary[],
): ClientSummary[] {
  const sessionByMac = new Map(
    sessionClients
      .filter((client) => client.mac)
      .map((client) => [normalizeMac(client.mac), client]),
  );
  const deviceByMac = new Map(devices.map((device) => [device.mac, device]));

  return integrationClients.map((client) => {
    const mac = normalizeMac(client.macAddress ?? client.id);
    const session = sessionByMac.get(mac);
    const apName =
      session?.ap_mac !== undefined ? deviceByMac.get(normalizeMac(session.ap_mac))?.name ?? '-' : '-';
    return {
      id: entityId(client.id),
      mac,
      name: session?.name ?? session?.hostname ?? client.name,
      ip: client.ipAddress ?? session?.ip ?? '-',
      network: session?.network ?? 'Unknown',
      apName,
      signalDbm: session?.signal ?? -90,
      experiencePct: round1(session?.satisfaction ?? 0),
      trafficMbps: round1((session?.tx_rate ?? 0 + (session?.rx_rate ?? 0)) / 1_000),
      roaming: false,
    };
  });
}

function mapNetworks(
  integrationNetworks: IntegrationNetwork[],
  clients: ClientSummary[],
): NetworkSummary[] {
  return integrationNetworks.map((network) => {
    const attachedClients = clients.filter((client) => client.network === network.name).length;
    return {
      id: entityId(network.id),
      name: network.name,
      purpose: mapNetworkPurpose(network.management),
      vlan: Number.isFinite(network.vlanId) ? network.vlanId : null,
      subnet: inferSubnet(network.metadata),
      clients: attachedClients,
      healthPct: attachedClients > 0 ? 90 + Math.min(attachedClients / 10, 8) : 99.9,
    };
  });
}

function mapEvents(events: SessionEvent[]): EventSummary[] {
  return events.slice(0, 20).map((event) => ({
    id: entityId(event._id),
    timestamp: event.datetime ?? new Date().toISOString(),
    category: mapEventCategory(event.subsystem),
    severity: mapEventSeverity(event.key, event.msg),
    title: event.key ?? 'event',
    detail: event.msg ?? 'No event detail provided.',
  }));
}

function mapDeviceStatus(integrationState: string, sessionState?: number): DeviceSummary['status'] {
  if (integrationState.toLowerCase() === 'offline' || sessionState === 0) {
    return 'offline';
  }

  if (integrationState.toLowerCase() === 'pending' || sessionState === 4 || sessionState === 5) {
    return 'degraded';
  }

  return 'online';
}

function mapNetworkPurpose(value: string): NetworkSummary['purpose'] {
  const normalized = value.toLowerCase();
  if (normalized.includes('guest')) return 'guest';
  if (normalized.includes('wan')) return 'wan';
  if (normalized.includes('vlan') || normalized.includes('corporate')) return 'corporate';
  return 'iot';
}

function inferSubnet(metadata?: Record<string, unknown>): string {
  const host = typeof metadata?.['hostIpAddress'] === 'string' ? metadata['hostIpAddress'] : undefined;
  const prefix = typeof metadata?.['prefixLength'] === 'number' ? metadata['prefixLength'] : undefined;
  if (host && prefix !== undefined) {
    return `${host}/${prefix}`;
  }

  return 'n/a';
}

function mapEventCategory(subsystem?: string): EventSummary['category'] {
  const normalized = subsystem?.toLowerCase() ?? '';
  if (normalized.includes('device')) return 'device';
  if (normalized.includes('user') || normalized.includes('client')) return 'client';
  if (normalized.includes('network') || normalized.includes('wan')) return 'network';
  return 'system';
}

function mapEventSeverity(key?: string, msg?: string): EventSummary['severity'] {
  const source = `${key ?? ''} ${msg ?? ''}`.toLowerCase();
  if (source.includes('fail') || source.includes('lost') || source.includes('offline')) {
    return 'error';
  }
  if (source.includes('warn') || source.includes('high') || source.includes('retry')) {
    return 'warn';
  }
  return 'info';
}

function inferGatewayLatency(events: EventSummary[]): number {
  return events.some((event) => event.severity === 'error') ? 24 : 11;
}

function inferPacketLoss(events: EventSummary[]): number {
  return events.some((event) => event.severity === 'error') ? 0.4 : 0.1;
}

function bytesToMbps(value?: number): number {
  return round1(((value ?? 0) * 8) / 1_000_000);
}

function round1(value: number): number {
  return Math.round(value * 10) / 10;
}
