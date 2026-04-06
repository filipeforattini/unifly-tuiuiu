import { entityId, normalizeMac } from '../domain/identity.js';
import type {
  ClientSummary,
  ControllerConfig,
  ControllerSnapshot,
  DeviceSummary,
  EventSummary,
  NetworkSummary,
} from '../domain/types.js';

export interface DemoSeedData {
  devices: DeviceSummary[];
  clients: ClientSummary[];
  networks: NetworkSummary[];
  eventTemplates: Omit<EventSummary, 'id' | 'timestamp'>[];
}

export function createDemoConfig(): ControllerConfig {
  return {
    controllerUrl: 'https://demo-controller.local',
    site: 'default',
    auth: { kind: 'apiKey', apiKey: 'demo-key' },
    tlsMode: 'accept-invalid',
    refreshIntervalMs: 1_200,
    websocketEnabled: true,
  };
}

export function createSeedData(): DemoSeedData {
  const devices: DeviceSummary[] = [
    {
      id: entityId('550e8400-e29b-41d4-a716-446655440000'),
      mac: normalizeMac('18:E8:29:AA:00:01'),
      name: 'Gateway Dream Machine',
      model: 'UDM Pro',
      ip: '10.0.0.1',
      status: 'online',
      cpuPct: 23,
      memPct: 48,
      clients: 68,
      uplink: 'WAN1 1G',
      txMbps: 220,
      rxMbps: 410,
    },
    {
      id: entityId('550e8400-e29b-41d4-a716-446655440001'),
      mac: normalizeMac('18:E8:29:AA:00:02'),
      name: 'Core Switch',
      model: 'USW Pro 24',
      ip: '10.0.0.2',
      status: 'online',
      cpuPct: 18,
      memPct: 31,
      clients: 52,
      uplink: 'SFP+ 10G',
      txMbps: 160,
      rxMbps: 210,
    },
    {
      id: entityId('507f1f77bcf86cd799439011'),
      mac: normalizeMac('18:E8:29:AA:00:03'),
      name: 'AP Lobby',
      model: 'U7 Pro',
      ip: '10.0.10.11',
      status: 'online',
      cpuPct: 34,
      memPct: 58,
      clients: 31,
      uplink: 'PoE GbE',
      txMbps: 94,
      rxMbps: 84,
    },
    {
      id: entityId('507f1f77bcf86cd799439012'),
      mac: normalizeMac('18:E8:29:AA:00:04'),
      name: 'AP Warehouse',
      model: 'U6 Enterprise',
      ip: '10.0.20.14',
      status: 'degraded',
      cpuPct: 61,
      memPct: 74,
      clients: 19,
      uplink: 'PoE GbE',
      txMbps: 77,
      rxMbps: 63,
    },
    {
      id: entityId('507f1f77bcf86cd799439013'),
      mac: normalizeMac('18:E8:29:AA:00:05'),
      name: 'Camera Switch',
      model: 'USW Lite 8 PoE',
      ip: '10.0.30.9',
      status: 'offline',
      cpuPct: 0,
      memPct: 0,
      clients: 0,
      uplink: 'PoE GbE',
      txMbps: 0,
      rxMbps: 0,
    },
  ];

  const clients: ClientSummary[] = [
    {
      id: entityId('507f1f77bcf86cd799439101'),
      mac: normalizeMac('A0:B1:C2:D3:E4:F1'),
      name: 'MacBook-Pro-OPS',
      ip: '10.0.10.31',
      network: 'Corp LAN',
      apName: 'AP Lobby',
      signalDbm: -56,
      experiencePct: 97.4,
      trafficMbps: 22.8,
      roaming: false,
    },
    {
      id: entityId('507f1f77bcf86cd799439102'),
      mac: normalizeMac('A0:B1:C2:D3:E4:F2'),
      name: 'Warehouse Scanner 14',
      ip: '10.0.20.87',
      network: 'IoT',
      apName: 'AP Warehouse',
      signalDbm: -68,
      experiencePct: 84.1,
      trafficMbps: 3.6,
      roaming: true,
    },
    {
      id: entityId('507f1f77bcf86cd799439103'),
      mac: normalizeMac('A0:B1:C2:D3:E4:F3'),
      name: 'Pixel 9 - Ana',
      ip: '10.0.10.48',
      network: 'Guest',
      apName: 'AP Lobby',
      signalDbm: -61,
      experiencePct: 91.7,
      trafficMbps: 5.4,
      roaming: false,
    },
    {
      id: entityId('507f1f77bcf86cd799439104'),
      mac: normalizeMac('A0:B1:C2:D3:E4:F4'),
      name: 'Apple TV Boardroom',
      ip: '10.0.10.90',
      network: 'Corp LAN',
      apName: 'AP Lobby',
      signalDbm: -52,
      experiencePct: 98.3,
      trafficMbps: 14.2,
      roaming: false,
    },
    {
      id: entityId('507f1f77bcf86cd799439105'),
      mac: normalizeMac('A0:B1:C2:D3:E4:F5'),
      name: 'CNC Gateway',
      ip: '10.0.20.12',
      network: 'IoT',
      apName: 'AP Warehouse',
      signalDbm: -72,
      experiencePct: 80.2,
      trafficMbps: 1.8,
      roaming: false,
    },
  ];

  const networks: NetworkSummary[] = [
    {
      id: entityId('550e8400-e29b-41d4-a716-446655441001'),
      name: 'Corp LAN',
      purpose: 'corporate',
      vlan: 10,
      subnet: '10.0.10.0/24',
      clients: 47,
      healthPct: 96.4,
    },
    {
      id: entityId('550e8400-e29b-41d4-a716-446655441002'),
      name: 'Guest',
      purpose: 'guest',
      vlan: 20,
      subnet: '10.0.20.0/24',
      clients: 22,
      healthPct: 91.1,
    },
    {
      id: entityId('550e8400-e29b-41d4-a716-446655441003'),
      name: 'IoT',
      purpose: 'iot',
      vlan: 30,
      subnet: '10.0.30.0/24',
      clients: 38,
      healthPct: 88.7,
    },
    {
      id: entityId('550e8400-e29b-41d4-a716-446655441004'),
      name: 'WAN1',
      purpose: 'wan',
      vlan: null,
      subnet: 'dhcp',
      clients: 0,
      healthPct: 99.9,
    },
  ];

  const eventTemplates: Omit<EventSummary, 'id' | 'timestamp'>[] = [
    {
      category: 'client',
      severity: 'info',
      title: 'Client roam detected',
      detail: 'Warehouse Scanner 14 moved from AP Warehouse to AP Lobby.',
    },
    {
      category: 'device',
      severity: 'warn',
      title: 'High CPU on AP Warehouse',
      detail: 'Radio optimization task is saturating CPU above 85%.',
    },
    {
      category: 'network',
      severity: 'info',
      title: 'Guest throughput spike',
      detail: 'Traffic burst detected on Guest network during visitor peak.',
    },
    {
      category: 'system',
      severity: 'error',
      title: 'Camera Switch unreachable',
      detail: 'PoE switch on camera segment stopped responding to health checks.',
    },
  ];

  return { devices, clients, networks, eventTemplates };
}

export function createInitialSnapshot(seed: DemoSeedData): ControllerSnapshot {
  const now = new Date().toISOString();
  return {
    connectionState: 'disconnected',
    lastRefreshAt: null,
    lastEventAt: now,
    demoPulseEnabled: true,
    runtime: {
      appMode: 'demo',
      dataSource: 'demo',
      controllerUrl: 'https://demo-controller.local',
      site: 'default',
      statusMessage: 'demo controller ready',
      lastError: null,
    },
    devices: seed.devices,
    clients: seed.clients,
    networks: seed.networks,
    events: seed.eventTemplates.slice(0, 3).map((event, index) => ({
      ...event,
      id: entityId(`evt-seed-${index}`),
      timestamp: now,
    })),
    metrics: {
      activeClients: seed.clients.length,
      onlineDevices: seed.devices.filter((device) => device.status === 'online').length,
      networks: seed.networks.length,
      eventsLastHour: 14,
      totalTxMbps: seed.devices.reduce((sum, device) => sum + device.txMbps, 0),
      totalRxMbps: seed.devices.reduce((sum, device) => sum + device.rxMbps, 0),
      siteHealth: {
        gatewayLatencyMs: 12.4,
        wanUptimePct: 99.98,
        wifiExperiencePct: 95.6,
        packetLossPct: 0.14,
      },
      throughputHistory: Array.from({ length: 18 }, (_, index) => ({
        timestamp: new Date(Date.now() - (17 - index) * 10_000).toISOString(),
        txMbps: 320 + index * 7,
        rxMbps: 510 - index * 6,
      })),
    },
  };
}
