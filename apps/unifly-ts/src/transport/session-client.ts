import { JsonHttpClient } from './http.js';

interface SessionEnvelope<T> {
  meta: {
    rc: string;
    msg?: string;
  };
  data: T[];
}

export interface SessionSite {
  _id: string;
  name: string;
  desc?: string;
}

export interface SessionDevice {
  _id: string;
  mac: string;
  type: string;
  ip?: string;
  name?: string;
  model?: string;
  state?: number;
  num_sta?: number;
  tx_bytes?: number;
  rx_bytes?: number;
  uplink?: string;
  sys_stats?: {
    cpu?: string;
    mem_total?: number;
    mem_used?: number;
  };
}

export interface SessionClientEntry {
  _id: string;
  mac: string;
  hostname?: string;
  ip?: string;
  name?: string;
  signal?: number;
  satisfaction?: number;
  essid?: string;
  ap_mac?: string;
  network?: string;
  tx_rate?: number;
  rx_rate?: number;
}

export interface SessionEvent {
  _id: string;
  key?: string;
  msg?: string;
  datetime?: string;
  subsystem?: string;
}

export class SessionApiClient {
  readonly #http: JsonHttpClient;

  constructor(controllerUrl: string, apiKey: string) {
    this.#http = new JsonHttpClient({
      baseUrl: `${stripTrailingSlash(controllerUrl)}/proxy/network/`,
      defaultHeaders: {
        'x-api-key': apiKey,
      },
    });
  }

  async listSites(): Promise<SessionSite[]> {
    const response = await this.#http.get<SessionEnvelope<SessionSite>>('api/self/sites');
    return unwrapEnvelope(response);
  }

  async listDevices(site: string): Promise<SessionDevice[]> {
    const response = await this.#http.get<SessionEnvelope<SessionDevice>>(`api/s/${site}/stat/device`);
    return unwrapEnvelope(response);
  }

  async listClients(site: string): Promise<SessionClientEntry[]> {
    const response = await this.#http.get<SessionEnvelope<SessionClientEntry>>(`api/s/${site}/stat/sta`);
    return unwrapEnvelope(response);
  }

  async listEvents(site: string, limit = 20): Promise<SessionEvent[]> {
    const response = await this.#http.get<SessionEnvelope<SessionEvent>>(`api/s/${site}/stat/event`, {
      _limit: limit,
    });
    return unwrapEnvelope(response);
  }
}

function unwrapEnvelope<T>(response: SessionEnvelope<T>): T[] {
  if (response.meta.rc !== 'ok') {
    throw new Error(response.meta.msg ?? 'session API returned a non-ok rc');
  }

  return response.data;
}

function stripTrailingSlash(value: string): string {
  return value.endsWith('/') ? value.slice(0, -1) : value;
}
