import { describe, expect, it } from 'vitest';

import { loadAppConfig } from '../src/config.js';

describe('loadAppConfig', () => {
  it('defaults to demo mode', () => {
    const config = loadAppConfig({});
    expect(config.mode).toBe('demo');
    expect(config.controller.site).toBe('default');
  });

  it('builds real hybrid config from env', () => {
    const config = loadAppConfig({
      UNIFLY_TS_MODE: 'real',
      UNIFI_CONTROLLER: 'https://192.168.1.1',
      UNIFI_SITE: 'lab',
      UNIFI_API_KEY: 'abc',
      UNIFI_USERNAME: 'admin',
      UNIFI_PASSWORD: 'secret',
    });

    expect(config.mode).toBe('real');
    expect(config.controller.auth).toEqual({
      kind: 'hybrid',
      apiKey: 'abc',
      username: 'admin',
      password: 'secret',
    });
  });
});
