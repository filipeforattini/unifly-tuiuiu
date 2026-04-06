import type { AuthCredentials, ControllerConfig } from './domain/types.js';

export type AppMode = 'demo' | 'real';

export interface AppConfig {
  mode: AppMode;
  controller: ControllerConfig;
}

export function loadAppConfig(env: NodeJS.ProcessEnv = process.env): AppConfig {
  const mode = parseMode(env.UNIFLY_TS_MODE);
  const controllerUrl = env.UNIFI_CONTROLLER?.trim() || 'https://demo-controller.local';
  const site = env.UNIFI_SITE?.trim() || 'default';
  const tlsMode = parseTlsMode(env.UNIFLY_TS_TLS_MODE);
  const refreshIntervalMs = parsePositiveInt(env.UNIFLY_TS_REFRESH_MS, 5_000);
  const websocketEnabled = env.UNIFLY_TS_WS !== '0';
  const auth = resolveAuth(env);

  return {
    mode,
    controller: {
      controllerUrl,
      site,
      auth,
      tlsMode,
      refreshIntervalMs,
      websocketEnabled,
    },
  };
}

function parseMode(value?: string): AppMode {
  return value?.trim().toLowerCase() === 'real' ? 'real' : 'demo';
}

function parseTlsMode(value?: string): ControllerConfig['tlsMode'] {
  const normalized = value?.trim().toLowerCase();
  if (normalized === 'system') {
    return 'system';
  }

  if (normalized === 'custom-pem') {
    return 'custom-pem';
  }

  return 'accept-invalid';
}

function parsePositiveInt(raw: string | undefined, fallback: number): number {
  const parsed = Number(raw);
  return Number.isFinite(parsed) && parsed > 0 ? parsed : fallback;
}

function resolveAuth(env: NodeJS.ProcessEnv): AuthCredentials {
  const apiKey = env.UNIFI_API_KEY?.trim();
  const username = env.UNIFI_USERNAME?.trim();
  const password = env.UNIFI_PASSWORD?.trim();

  if (apiKey && username && password) {
    return { kind: 'hybrid', apiKey, username, password };
  }

  if (apiKey) {
    return { kind: 'apiKey', apiKey };
  }

  if (username && password) {
    return { kind: 'credentials', username, password };
  }

  return { kind: 'apiKey', apiKey: 'demo-key' };
}
