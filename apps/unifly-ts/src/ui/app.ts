import {
  Box,
  Footer,
  Gauge,
  Header,
  LineChart,
  Main,
  Screen,
  Spacer,
  Text,
  Title,
  darkTheme,
  render,
  setTheme,
  useApp,
  useEffect,
  useInput,
  useState,
} from 'tuiuiu.js';

import type { ControllerSnapshot } from '../domain/types.js';
import type { Controller } from '../runtime/controller.js';
import { formatPercent, formatThroughput, formatTimestamp, statusColor } from './format.js';

type ScreenId = 'dashboard' | 'devices' | 'clients' | 'networks' | 'events';

const SCREEN_ORDER: ScreenId[] = ['dashboard', 'devices', 'clients', 'networks', 'events'];

export async function launchApp(controller: Controller): Promise<void> {
  setTheme(darkTheme);
  await controller.connect();

  const { waitUntilExit } = render(() => App({ controller }), { fullHeight: true });
  await waitUntilExit();
  await controller.disconnect();
}

function App(props: { controller: Controller }) {
  const app = useApp();
  const [screen, setScreen] = useState<ScreenId>('dashboard');
  const [snapshot, setSnapshot] = useState<ControllerSnapshot>(props.controller.store.current());

  useEffect(() => props.controller.store.subscribe((nextSnapshot) => setSnapshot(nextSnapshot)));

  useInput((input, key) => {
    if (input === 'q' || key.escape) {
      app.exit();
      return;
    }

    if (input === 'r') {
      void props.controller.refresh();
      return;
    }

    if (input === 'd') {
      props.controller.toggleDemoPulse();
      return;
    }

    if (['1', '2', '3', '4', '5'].includes(input)) {
      setScreen(SCREEN_ORDER[Number(input) - 1] ?? 'dashboard');
      return;
    }

    if (key.rightArrow || input === 'l') {
      setScreen(nextScreen(screen(), 1));
      return;
    }

    if (key.leftArrow || input === 'h') {
      setScreen(nextScreen(screen(), -1));
    }
  });

  const current = snapshot();

  return Screen(
    {},
    Header(
      { backgroundColor: 'muted', width: 'fill', paddingX: 1 },
      Title('unifly-ts', { color: 'foreground' }),
      Text({ color: 'cyan' }, '  tuiuiu.js study'),
      Spacer(),
      HeaderBadge(connectionLabel(current.connectionState), connectionColor(current.connectionState)),
      Text({ color: 'mutedForeground' }, ' '),
      HeaderBadge(
        current.runtime.dataSource === 'unifi-live' ? 'LIVE' : current.runtime.dataSource.toUpperCase(),
        current.runtime.dataSource === 'unifi-live' ? 'magenta' : 'warning',
      ),
      Text({ color: 'mutedForeground' }, '  '),
      Text({ color: 'mutedForeground' }, `site ${current.runtime.site}`),
      Text({ color: 'mutedForeground' }, '  '),
      Text({ color: 'cyan' }, formatTimestamp(current.lastRefreshAt)),
    ),
    Main(
      { paddingX: 1 },
      Box(
        { flexDirection: 'column', gap: 1, height: 'fill' },
        ScreenTabs(screen()),
        HeroMetrics(current),
        ContentPanel(current, screen()),
      ),
    ),
    Footer(
      { backgroundColor: 'muted', width: 'fill', paddingX: 1 },
      Text({ color: 'success' }, '●'),
      Text({ color: 'mutedForeground' }, ` ${current.runtime.statusMessage}`),
      Spacer(),
      Text({ color: 'mutedForeground' }, '[1-5] views'),
      Text({ color: 'mutedForeground' }, '  [h/l] cycle'),
      Text({ color: 'mutedForeground' }, '  [r] refresh'),
      Text({ color: 'mutedForeground' }, '  [q] quit'),
    ),
  );
}

function ScreenTabs(activeScreen: ScreenId) {
  return Box(
    { flexDirection: 'row', gap: 2, paddingX: 1 },
    ...SCREEN_ORDER.map((screenId, index) => {
      const active = screenId === activeScreen;
      const label = `${index + 1} ${screenId.charAt(0).toUpperCase()}${screenId.slice(1)}`;
      return active
        ? Text({ color: 'magenta', bold: true, inverse: true }, ` ${label} `)
        : Text({ color: 'mutedForeground' }, label);
    }),
  );
}

function HeroMetrics(snapshot: ControllerSnapshot) {
  const m = snapshot.metrics;
  return Box(
    { flexDirection: 'row', gap: 2, paddingX: 1 },
    HeroItem('Clients', String(m.activeClients), 'cyan'),
    HeroItem('Online', String(m.onlineDevices), 'green'),
    HeroItem('TX', formatThroughput(m.totalTxMbps), 'yellow'),
    HeroItem('RX', formatThroughput(m.totalRxMbps), 'magenta'),
    Text({ color: 'mutedForeground' }, `  lat ${m.siteHealth.gatewayLatencyMs.toFixed(1)}ms`),
    Text({ color: 'mutedForeground' }, `  loss ${formatPercent(m.siteHealth.packetLossPct)}`),
  );
}

function HeroItem(label: string, value: string, color: string) {
  return Box(
    { flexDirection: 'row', gap: 1 },
    Text({ color: 'mutedForeground' }, `${label}:`),
    Text({ color, bold: true }, value),
  );
}

function ContentPanel(snapshot: ControllerSnapshot, activeScreen: ScreenId) {
  if (activeScreen === 'dashboard') {
    return DashboardScreen(snapshot);
  }
  if (activeScreen === 'devices') {
    return DevicesScreen(snapshot);
  }
  if (activeScreen === 'clients') {
    return ClientsScreen(snapshot);
  }
  if (activeScreen === 'networks') {
    return NetworksScreen(snapshot);
  }
  return EventsScreen(snapshot);
}

function DashboardScreen(snapshot: ControllerSnapshot) {
  const chartData = snapshot.metrics.throughputHistory.map((point) => point.txMbps);
  const rxChartData = snapshot.metrics.throughputHistory.map((point) => point.rxMbps);

  return Box(
    { flexDirection: 'column', gap: 1, height: 'fill' },
    CompactPanel(
      { label: 'WAN Traffic', height: 9 },
      LineChart({
        series: [
          { name: 'TX', data: chartData, color: 'cyan' },
          { name: 'RX', data: rxChartData, color: 'magenta' },
        ],
        height: 6,
        showLegend: true,
      }),
    ),
    Box(
      { flexDirection: 'row', gap: 1, minHeight: 8 },
      RuntimeCard(snapshot),
      HealthCard(snapshot),
      FocusCard(snapshot),
    ),
    Box(
      { flexDirection: 'row', gap: 1, minHeight: 10 },
      NetworksCard(snapshot),
      ClientsHeatCard(snapshot),
      DevicesHeatCard(snapshot),
    ),
    CompactPanel(
      { label: 'Recent Events', height: 'fill' },
      Box(
        { flexDirection: 'column', paddingX: 1 },
        ...snapshot.events.slice(0, 8).map((event) =>
          Text(
            { color: statusColor(event.severity) },
            `${formatTimestamp(event.timestamp)} ${pad(event.category.toUpperCase(), 7)} ${truncate(event.title, 18)} ${truncate(event.detail, 70)}`,
          ),
        ),
      ),
    ),
  );
}

function RuntimeCard(snapshot: ControllerSnapshot) {
  return CompactPanel(
    { label: 'Runtime', flexGrow: 1, height: 6 },
    Box(
      { flexDirection: 'column', paddingX: 1 },
      Text({ color: 'foreground' }, `${snapshot.runtime.appMode} / ${snapshot.runtime.dataSource}`),
      Text({ color: 'mutedForeground' }, truncate(snapshot.runtime.controllerUrl, 26)),
      Text({ color: 'mutedForeground' }, `site ${snapshot.runtime.site}`),
      Text(
        { color: snapshot.runtime.lastError ? 'error' : 'mutedForeground' },
        truncate(snapshot.runtime.lastError ?? snapshot.runtime.statusMessage, 34),
      ),
    ),
  );
}

function HealthCard(snapshot: ControllerSnapshot) {
  return CompactPanel(
    { label: 'Health', flexGrow: 1, height: 6 },
    Box(
      { flexDirection: 'column', paddingX: 1 },
      Gauge({
        value: snapshot.metrics.siteHealth.wifiExperiencePct,
        max: 100,
        style: 'linear',
        label: `WiFi ${formatPercent(snapshot.metrics.siteHealth.wifiExperiencePct)}`,
      }),
      Gauge({
        value: snapshot.metrics.siteHealth.wanUptimePct,
        max: 100,
        style: 'linear',
        label: `WAN ${formatPercent(snapshot.metrics.siteHealth.wanUptimePct)}`,
      }),
      Text({ color: 'mutedForeground' }, `lat ${snapshot.metrics.siteHealth.gatewayLatencyMs.toFixed(1)} ms`),
    ),
  );
}

function FocusCard(snapshot: ControllerSnapshot) {
  return CompactPanel(
    { label: 'Focus', flexGrow: 1, height: 6 },
    Box(
      { flexDirection: 'column', paddingX: 1 },
      Text({ color: 'foreground' }, 'Snapshot-driven UI'),
      Text({ color: 'foreground' }, 'Full-width dashboard'),
      Text({ color: 'foreground' }, 'TS runtime isolated from view'),
      Text({ color: 'mutedForeground' }, truncate(snapshot.runtime.statusMessage, 34)),
    ),
  );
}

function NetworksCard(snapshot: ControllerSnapshot) {
  return CompactPanel(
    { label: 'Networks', flexGrow: 1, height: 7 },
    Box(
      { flexDirection: 'column', paddingX: 1 },
      ...snapshot.networks.slice(0, 5).map((network) =>
        Text(
          { color: network.healthPct >= 95 ? 'success' : network.healthPct >= 85 ? 'warning' : 'error' },
          `${pad(truncate(network.name, 12), 12)} v${pad(network.vlan === null ? '-' : String(network.vlan), 4)} c${pad(String(network.clients), 3)} ${network.healthPct.toFixed(0)}%`,
        ),
      ),
    ),
  );
}

function ClientsHeatCard(snapshot: ControllerSnapshot) {
  return CompactPanel(
    { label: 'Top Clients', flexGrow: 1, height: 7 },
    Box(
      { flexDirection: 'column', paddingX: 1 },
      ...snapshot.clients
        .slice()
        .sort((left, right) => right.trafficMbps - left.trafficMbps)
        .slice(0, 5)
        .map((client) =>
          Text(
            { color: client.roaming ? 'warning' : 'foreground' },
            `${pad(truncate(client.name, 18), 18)} ${pad(client.trafficMbps.toFixed(1), 5)}M ${pad(client.experiencePct.toFixed(0), 3)}%`,
          ),
        ),
    ),
  );
}

function DevicesHeatCard(snapshot: ControllerSnapshot) {
  return CompactPanel(
    { label: 'Devices', flexGrow: 1, height: 7 },
    Box(
      { flexDirection: 'column', paddingX: 1 },
      ...snapshot.devices.slice(0, 5).map((device) =>
        Text(
          { color: statusColor(device.status) },
          `${pad(truncate(device.name, 16), 16)} c${pad(device.cpuPct.toFixed(0), 3)} m${pad(device.memPct.toFixed(0), 3)} u${pad(String(device.clients), 3)}`,
        ),
      ),
    ),
  );
}

function DevicesScreen(snapshot: ControllerSnapshot) {
  return CompactPanel(
    { label: `Devices (${snapshot.devices.length})`, height: 'fill' },
    Box(
      { flexDirection: 'column', paddingX: 1 },
      Text({ color: 'mutedForeground' }, 'Sta Name               Model        IP            CPU Mem Cli TX   RX'),
      ...snapshot.devices.map((device) =>
        Text(
          { color: statusColor(device.status) },
          `${dot(device.status)}  ${pad(truncate(device.name, 18), 18)} ${pad(device.model, 12)} ${pad(device.ip, 13)} ${pad(device.cpuPct.toFixed(0), 3)} ${pad(device.memPct.toFixed(0), 3)} ${pad(String(device.clients), 3)} ${pad(device.txMbps.toFixed(0), 4)} ${pad(device.rxMbps.toFixed(0), 4)}`,
        ),
      ),
    ),
  );
}

function ClientsScreen(snapshot: ControllerSnapshot) {
  return CompactPanel(
    { label: `Clients (${snapshot.clients.length})`, height: 'fill' },
    Box(
      { flexDirection: 'column', paddingX: 1 },
      Text({ color: 'mutedForeground' }, 'Typ Name                 IP            AP           Sig Exp TX/RX'),
      ...snapshot.clients.map((client) =>
        Text(
          { color: client.roaming ? 'warning' : 'foreground' },
          `${client.network === 'Unknown' ? 'E' : 'W'}   ${pad(truncate(client.name, 20), 20)} ${pad(client.ip, 13)} ${pad(truncate(client.apName, 10), 10)} ${pad(String(client.signalDbm), 4)} ${pad(client.experiencePct.toFixed(0), 3)} ${pad(client.trafficMbps.toFixed(1), 5)}M`,
        ),
      ),
    ),
  );
}

function NetworksScreen(snapshot: ControllerSnapshot) {
  return CompactPanel(
    { label: `Networks (${snapshot.networks.length})`, height: 'fill' },
    Box(
      { flexDirection: 'column', paddingX: 1 },
      Text({ color: 'mutedForeground' }, 'Name         VLAN Gateway           Clients Health Purpose'),
      ...snapshot.networks.map((network) =>
        Text(
          { color: network.healthPct >= 95 ? 'success' : network.healthPct >= 85 ? 'warning' : 'error' },
          `${pad(truncate(network.name, 12), 12)} ${pad(network.vlan === null ? '-' : String(network.vlan), 4)} ${pad(truncate(network.subnet, 16), 16)} ${pad(String(network.clients), 6)} ${pad(network.healthPct.toFixed(0), 4)}% ${network.purpose}`,
        ),
      ),
    ),
  );
}

function EventsScreen(snapshot: ControllerSnapshot) {
  return CompactPanel(
    { label: 'Events', height: 'fill' },
    Box(
      { flexDirection: 'column', paddingX: 1 },
      ...snapshot.events.map((event) =>
        Text(
          { color: statusColor(event.severity) },
          `${formatTimestamp(event.timestamp)} ${pad(event.category.toUpperCase(), 7)} ${pad(event.severity.toUpperCase(), 5)} ${truncate(event.title, 18)} ${truncate(event.detail, 78)}`,
        ),
      ),
    ),
  );
}

function HeaderBadge(label: string, color: string) {
  return Text({ color, bold: true }, label);
}

function CompactPanel(
  options: {
    label: string;
    height?: number | 'fill';
    width?: number;
    flexGrow?: number;
  },
  ...children: ReturnType<typeof Box>[]
) {
  const panelProps = {
      borderText: options.label,
      borderTextAlign: 'left',
      borderStyle: 'round',
      borderColor: 'muted',
      padding: 0,
      flexDirection: 'column',
      height: options.height,
      width: options.width,
      flexGrow: options.flexGrow,
    } as const;

  // `tuiuiu.js` runtime supports `borderText`, but the published d.ts does not expose it yet.
  return Box(panelProps as any, ...children);
}

function nextScreen(current: ScreenId, delta: 1 | -1): ScreenId {
  const index = SCREEN_ORDER.indexOf(current);
  const nextIndex = (index + delta + SCREEN_ORDER.length) % SCREEN_ORDER.length;
  return SCREEN_ORDER[nextIndex] ?? 'dashboard';
}

function pad(value: string, size: number): string {
  return value.padEnd(size, ' ');
}

function truncate(value: string, size: number): string {
  return value.length <= size ? value : `${value.slice(0, Math.max(0, size - 1))}…`;
}

function dot(status: ControllerSnapshot['devices'][number]['status']): string {
  if (status === 'online') return '●';
  if (status === 'degraded') return '◐';
  return '○';
}

function connectionColor(state: ControllerSnapshot['connectionState']): string {
  if (state === 'connected') return 'success';
  if (state === 'failed') return 'error';
  if (state === 'connecting' || state === 'reconnecting') return 'warning';
  return 'mutedForeground';
}

function connectionLabel(state: ControllerSnapshot['connectionState']): string {
  return state.toUpperCase();
}
