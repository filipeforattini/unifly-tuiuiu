export type ConnectionState =
  | 'disconnected'
  | 'connecting'
  | 'connected'
  | 'reconnecting'
  | 'failed';

export type EntityId =
  | { kind: 'uuid'; value: string }
  | { kind: 'legacy'; value: string };

export type AuthCredentials =
  | { kind: 'apiKey'; apiKey: string }
  | { kind: 'credentials'; username: string; password: string }
  | { kind: 'hybrid'; apiKey: string; username: string; password: string };

export interface ControllerConfig {
  controllerUrl: string;
  site: string;
  auth: AuthCredentials;
  tlsMode: 'system' | 'accept-invalid' | 'custom-pem';
  refreshIntervalMs: number;
  websocketEnabled: boolean;
}

export interface DeviceSummary {
  id: EntityId;
  mac: string;
  name: string;
  model: string;
  ip: string;
  status: 'online' | 'offline' | 'degraded';
  cpuPct: number;
  memPct: number;
  clients: number;
  uplink: string;
  txMbps: number;
  rxMbps: number;
}

export interface ClientSummary {
  id: EntityId;
  mac: string;
  name: string;
  ip: string;
  network: string;
  apName: string;
  signalDbm: number;
  experiencePct: number;
  trafficMbps: number;
  roaming: boolean;
}

export interface NetworkSummary {
  id: EntityId;
  name: string;
  purpose: 'corporate' | 'guest' | 'iot' | 'wan';
  vlan: number | null;
  subnet: string;
  clients: number;
  healthPct: number;
}

export interface EventSummary {
  id: EntityId;
  timestamp: string;
  category: 'device' | 'client' | 'network' | 'system';
  severity: 'info' | 'warn' | 'error';
  title: string;
  detail: string;
}

export interface SiteHealth {
  gatewayLatencyMs: number;
  wanUptimePct: number;
  wifiExperiencePct: number;
  packetLossPct: number;
}

export interface ThroughputPoint {
  timestamp: string;
  txMbps: number;
  rxMbps: number;
}

export interface DashboardMetrics {
  activeClients: number;
  onlineDevices: number;
  networks: number;
  eventsLastHour: number;
  totalTxMbps: number;
  totalRxMbps: number;
  siteHealth: SiteHealth;
  throughputHistory: ThroughputPoint[];
}

export interface RuntimeMeta {
  appMode: 'demo' | 'real';
  dataSource: 'demo' | 'unifi-live' | 'fallback-demo';
  controllerUrl: string;
  site: string;
  statusMessage: string;
  lastError: string | null;
}

export interface ControllerSnapshot {
  connectionState: ConnectionState;
  lastRefreshAt: string | null;
  lastEventAt: string | null;
  demoPulseEnabled: boolean;
  runtime: RuntimeMeta;
  metrics: DashboardMetrics;
  devices: DeviceSummary[];
  clients: ClientSummary[];
  networks: NetworkSummary[];
  events: EventSummary[];
}
