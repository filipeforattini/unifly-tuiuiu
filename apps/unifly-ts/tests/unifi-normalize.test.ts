import { describe, expect, it } from 'vitest';

import { createInitialSnapshot, createSeedData } from '../src/services/demo-fixtures.js';
import { buildRealSnapshot } from '../src/services/unifi-normalize.js';

describe('buildRealSnapshot', () => {
  it('merges integration and session entities into the runtime snapshot', () => {
    const previous = createInitialSnapshot(createSeedData());
    const snapshot = buildRealSnapshot({
      integrationSite: { id: 'site-1', name: 'default', internalReference: 'default' },
      integrationDevices: [
        {
          id: '550e8400-e29b-41d4-a716-446655440010',
          macAddress: 'AA-BB-CC-DD-EE-FF',
          ipAddress: '10.0.0.2',
          name: 'Core Switch',
          model: 'USW Pro',
          state: 'ONLINE',
        },
      ],
      integrationClients: [
        {
          id: '550e8400-e29b-41d4-a716-446655440011',
          name: 'ops-mac',
          type: 'WIRELESS',
          ipAddress: '10.0.10.31',
          macAddress: '11-22-33-44-55-66',
        },
      ],
      integrationNetworks: [
        {
          id: '550e8400-e29b-41d4-a716-446655440012',
          name: 'Corp LAN',
          enabled: true,
          management: 'corporate',
          vlanId: 10,
          metadata: { hostIpAddress: '10.0.10.1', prefixLength: 24 },
        },
      ],
      sessionDevices: [
        {
          _id: '507f1f77bcf86cd799439011',
          mac: 'aa:bb:cc:dd:ee:ff',
          type: 'usw',
          ip: '10.0.0.2',
          name: 'Core Switch',
          model: 'USW Pro',
          state: 1,
          num_sta: 18,
          tx_bytes: 10_000_000,
          rx_bytes: 15_000_000,
          sys_stats: { cpu: '18', mem_total: 100, mem_used: 40 },
        },
      ],
      sessionClients: [
        {
          _id: '507f1f77bcf86cd799439012',
          mac: '11:22:33:44:55:66',
          name: 'ops-mac',
          ip: '10.0.10.31',
          signal: -55,
          satisfaction: 96,
          network: 'Corp LAN',
          ap_mac: 'aa:bb:cc:dd:ee:ff',
          tx_rate: 150_000,
          rx_rate: 80_000,
        },
      ],
      sessionEvents: [
        {
          _id: 'evt-1',
          key: 'EVT_GW_Lost_Contact',
          msg: 'Gateway lost contact briefly',
          subsystem: 'device',
          datetime: '2026-04-06T12:00:00.000Z',
        },
      ],
      previous,
    });

    expect(snapshot.connectionState).toBe('connected');
    expect(snapshot.devices[0]?.clients).toBe(18);
    expect(snapshot.clients[0]?.apName).toBe('Core Switch');
    expect(snapshot.networks[0]?.subnet).toBe('10.0.10.1/24');
    expect(snapshot.events[0]?.severity).toBe('error');
  });
});
