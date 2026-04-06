import { JsonHttpClient } from './http.js';

export interface IntegrationPage<T> {
  offset: number;
  limit: number;
  count: number;
  totalCount: number;
  data: T[];
}

export interface IntegrationSite {
  id: string;
  name: string;
  internalReference: string;
}

export interface IntegrationDevice {
  id: string;
  macAddress: string;
  ipAddress?: string;
  name: string;
  model: string;
  state: string;
}

export interface IntegrationClientEntity {
  id: string;
  name: string;
  type: string;
  ipAddress?: string;
  connectedAt?: string;
  macAddress?: string;
}

export interface IntegrationNetwork {
  id: string;
  name: string;
  enabled: boolean;
  management: string;
  vlanId: number;
  default?: boolean;
  metadata?: Record<string, unknown>;
}

export class IntegrationApiClient {
  readonly #http: JsonHttpClient;

  constructor(controllerUrl: string, apiKey: string) {
    this.#http = new JsonHttpClient({
      baseUrl: `${stripTrailingSlash(controllerUrl)}/proxy/network/integration/`,
      defaultHeaders: {
        'x-api-key': apiKey,
      },
    });
  }

  async listSites(offset = 0, limit = 100): Promise<IntegrationPage<IntegrationSite>> {
    return this.#http.get<IntegrationPage<IntegrationSite>>('v1/sites', { offset, limit });
  }

  async listDevices(siteId: string, offset = 0, limit = 200): Promise<IntegrationPage<IntegrationDevice>> {
    return this.#http.get<IntegrationPage<IntegrationDevice>>(`v1/sites/${siteId}/devices`, {
      offset,
      limit,
    });
  }

  async listClients(
    siteId: string,
    offset = 0,
    limit = 200,
  ): Promise<IntegrationPage<IntegrationClientEntity>> {
    return this.#http.get<IntegrationPage<IntegrationClientEntity>>(`v1/sites/${siteId}/clients`, {
      offset,
      limit,
    });
  }

  async listNetworks(
    siteId: string,
    offset = 0,
    limit = 200,
  ): Promise<IntegrationPage<IntegrationNetwork>> {
    return this.#http.get<IntegrationPage<IntegrationNetwork>>(`v1/sites/${siteId}/networks`, {
      offset,
      limit,
    });
  }
}

function stripTrailingSlash(value: string): string {
  return value.endsWith('/') ? value.slice(0, -1) : value;
}
