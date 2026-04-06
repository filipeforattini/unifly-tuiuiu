import {
  Box,
  Footer,
  Gauge,
  Header,
  LineChart,
  Main,
  Panel,
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
      Title('unifly-ts / tuiuiu.js lab', { color: 'foreground' }),
      Spacer(),
      Text({ color: 'mutedForeground' }, current.connectionState.toUpperCase()),
      Text({ color: 'mutedForeground' }, '  '),
      Text(
        { color: current.runtime.dataSource === 'unifi-live' ? 'success' : 'warning' },
        current.runtime.dataSource.toUpperCase(),
      ),
      Text({ color: 'mutedForeground' }, '  '),
      Text({ color: 'cyan' }, `refresh ${formatTimestamp(current.lastRefreshAt)}`),
    ),
    Main(
      { padding: 1 },
      Box(
        { flexDirection: 'row', gap: 1, height: 'fill' },
        NavigationPanel(screen()),
        ContentPanel(current, screen()),
      ),
    ),
    Footer(
      { backgroundColor: 'muted', width: 'fill', paddingX: 1 },
      Text({ color: 'mutedForeground' }, '[1-5] Screens'),
      Text({ color: 'mutedForeground' }, '  [R] Refresh'),
      Text({ color: 'mutedForeground' }, '  [D] Demo pulse'),
      Spacer(),
      Text(
        { color: current.demoPulseEnabled ? 'success' : 'warning' },
        current.demoPulseEnabled ? 'Pulse on' : 'Pulse paused',
      ),
      Text({ color: 'mutedForeground' }, '  [Q] Quit'),
    ),
  );
}

function NavigationPanel(activeScreen: ScreenId) {
  return Panel(
    { title: 'Views', width: 28, height: 'fill' },
    Box(
      { flexDirection: 'column', gap: 1 },
      ...SCREEN_ORDER.map((screenId, index) =>
        Text(
          { color: activeScreen === screenId ? 'cyan' : 'foreground', bold: activeScreen === screenId },
          `${index + 1}. ${screenId}`,
        ),
      ),
      Text({ color: 'mutedForeground' }, ''),
      Text({ color: 'mutedForeground' }, 'TUI-first proving ground for tuiuiu.js.'),
      Text({ color: 'mutedForeground' }, 'Runtime/store contracts stay isolated from the view.'),
    ),
  );
}

function ContentPanel(snapshot: ControllerSnapshot, activeScreen: ScreenId) {
  return Box(
    { flexDirection: 'column', flexGrow: 1, gap: 1, height: 'fill' },
    HeroMetrics(snapshot),
    activeScreen === 'dashboard'
      ? DashboardScreen(snapshot)
      : activeScreen === 'devices'
        ? DevicesScreen(snapshot)
        : activeScreen === 'clients'
          ? ClientsScreen(snapshot)
          : activeScreen === 'networks'
            ? NetworksScreen(snapshot)
            : EventsScreen(snapshot),
  );
}

function HeroMetrics(snapshot: ControllerSnapshot) {
  const metrics = snapshot.metrics;
  return Box(
    { flexDirection: 'row', gap: 1, minHeight: 10 },
    MetricPanel('Clients', String(metrics.activeClients), 'cyan'),
    MetricPanel('Devices', String(metrics.onlineDevices), 'green'),
    MetricPanel('TX', formatThroughput(metrics.totalTxMbps), 'yellow'),
    MetricPanel('RX', formatThroughput(metrics.totalRxMbps), 'magenta'),
  );
}

function DashboardScreen(snapshot: ControllerSnapshot) {
  const chartData = snapshot.metrics.throughputHistory.map((point) => point.txMbps);
  const rxChartData = snapshot.metrics.throughputHistory.map((point) => point.rxMbps);

  return Box(
    { flexDirection: 'row', gap: 1, height: 'fill' },
    Box(
      { flexDirection: 'column', flexGrow: 2, gap: 1 },
      Panel(
        { title: 'Throughput timeline', height: 16 },
        LineChart({
          series: [
            { name: 'TX', data: chartData, color: 'cyan' },
            { name: 'RX', data: rxChartData, color: 'magenta' },
          ],
          height: 12,
          showLegend: true,
        }),
      ),
      Panel(
        { title: 'Recent events', height: 'fill' },
        Box(
          { flexDirection: 'column', gap: 1 },
          ...snapshot.events.slice(0, 6).map((event) =>
            Text(
              { color: statusColor(event.severity) },
              `${event.category.toUpperCase()}  ${event.title}  ${event.detail}`,
            ),
          ),
        ),
      ),
    ),
    Box(
      { flexDirection: 'column', flexGrow: 1, gap: 1 },
      Panel(
        { title: 'Runtime', height: 10 },
        Box(
          { flexDirection: 'column', gap: 1 },
          Text({}, `Mode: ${snapshot.runtime.appMode}`),
          Text({}, `Source: ${snapshot.runtime.dataSource}`),
          Text({}, `Controller: ${snapshot.runtime.controllerUrl}`),
          Text({}, `Site: ${snapshot.runtime.site}`),
          Text(
            { color: snapshot.runtime.lastError ? 'error' : 'mutedForeground' },
            snapshot.runtime.lastError ?? snapshot.runtime.statusMessage,
          ),
        ),
      ),
      Panel(
        { title: 'Site health', height: 16 },
        Box(
          { flexDirection: 'column', gap: 1 },
          Gauge({
            value: snapshot.metrics.siteHealth.wifiExperiencePct,
            max: 100,
            style: 'linear',
            label: `WiFi experience ${formatPercent(snapshot.metrics.siteHealth.wifiExperiencePct)}`,
          }),
          Gauge({
            value: snapshot.metrics.siteHealth.wanUptimePct,
            max: 100,
            style: 'linear',
            label: `WAN uptime ${formatPercent(snapshot.metrics.siteHealth.wanUptimePct)}`,
          }),
          Text({}, `Gateway latency: ${snapshot.metrics.siteHealth.gatewayLatencyMs.toFixed(1)} ms`),
          Text({}, `Packet loss: ${formatPercent(snapshot.metrics.siteHealth.packetLossPct)}`),
        ),
      ),
      Panel(
        { title: 'Intent', height: 'fill' },
        Box(
          { flexDirection: 'column', gap: 1 },
          Text({ color: 'foreground' }, 'The TS runtime is separated from the view.'),
          Text({ color: 'foreground' }, 'This makes it easier to fan out into richer terminal workflows.'),
          Text({ color: 'mutedForeground' }, snapshot.runtime.statusMessage),
        ),
      ),
    ),
  );
}

function DevicesScreen(snapshot: ControllerSnapshot) {
  return Panel(
    { title: 'Devices', height: 'fill' },
    Box(
      { flexDirection: 'column', gap: 1 },
      ...snapshot.devices.map((device) =>
        Text(
          { color: statusColor(device.status) },
          `${pad(device.name, 22)} ${pad(device.model, 14)} ${pad(device.ip, 14)} ` +
            `CPU ${device.cpuPct.toFixed(0)}%  MEM ${device.memPct.toFixed(0)}%  ` +
            `Clients ${pad(String(device.clients), 3)}  TX ${pad(device.txMbps.toFixed(1), 6)}  ` +
            `RX ${pad(device.rxMbps.toFixed(1), 6)}  ${device.status.toUpperCase()}`,
        ),
      ),
    ),
  );
}

function ClientsScreen(snapshot: ControllerSnapshot) {
  return Panel(
    { title: 'Clients', height: 'fill' },
    Box(
      { flexDirection: 'column', gap: 1 },
      ...snapshot.clients.map((client) =>
        Text(
          { color: client.roaming ? 'warning' : 'foreground' },
          `${pad(client.name, 24)} ${pad(client.network, 10)} ${pad(client.apName, 14)} ` +
            `${pad(client.ip, 14)} signal ${pad(String(client.signalDbm), 4)}  ` +
            `exp ${pad(client.experiencePct.toFixed(1), 5)}  ` +
            `traffic ${pad(client.trafficMbps.toFixed(1), 5)} Mbps` +
            `${client.roaming ? '  ROAM' : ''}`,
        ),
      ),
    ),
  );
}

function NetworksScreen(snapshot: ControllerSnapshot) {
  return Box(
    { flexDirection: 'row', gap: 1, height: 'fill' },
    Panel(
      { title: 'Networks', flexGrow: 2, height: 'fill' },
      Box(
        { flexDirection: 'column', gap: 1 },
        ...snapshot.networks.map((network) =>
          Text(
            { color: network.healthPct >= 95 ? 'success' : network.healthPct >= 85 ? 'warning' : 'error' },
            `${pad(network.name, 12)} ${pad(network.purpose, 10)} ${pad(network.subnet, 18)} ` +
              `VLAN ${pad(network.vlan === null ? '-' : String(network.vlan), 4)} ` +
              `Clients ${pad(String(network.clients), 4)} ` +
              `Health ${network.healthPct.toFixed(1)}%`,
          ),
        ),
      ),
    ),
    Panel(
      { title: 'Why this architecture', flexGrow: 1, height: 'fill' },
      Box(
        { flexDirection: 'column', gap: 1 },
        Text({}, 'The domain model is now explicit in TS.'),
        Text({}, 'Dual-API merge logic can land in the runtime without leaking transport quirks into the UI.'),
        Text({ color: 'mutedForeground' }, `Last event: ${formatTimestamp(snapshot.lastEventAt)}`),
      ),
    ),
  );
}

function EventsScreen(snapshot: ControllerSnapshot) {
  return Panel(
    { title: 'Live events', height: 'fill' },
    Box(
      { flexDirection: 'column', gap: 1 },
      ...snapshot.events.map((event) =>
        Text(
          { color: statusColor(event.severity) },
          `${formatTimestamp(event.timestamp)}  ${pad(event.category.toUpperCase(), 8)}  ${pad(event.severity.toUpperCase(), 5)}  ${event.title}  ${event.detail}`,
        ),
      ),
    ),
  );
}

function MetricPanel(label: string, value: string, color: string) {
  return Panel(
    { title: label, flexGrow: 1, height: 10 },
    Box(
      { flexDirection: 'column', justifyContent: 'center', height: 'fill' },
      Text({ color, bold: true }, value),
    ),
  );
}

function nextScreen(current: ScreenId, delta: 1 | -1): ScreenId {
  const index = SCREEN_ORDER.indexOf(current);
  const nextIndex = (index + delta + SCREEN_ORDER.length) % SCREEN_ORDER.length;
  return SCREEN_ORDER[nextIndex] ?? 'dashboard';
}

function pad(value: string, size: number): string {
  return value.padEnd(size, ' ');
}
