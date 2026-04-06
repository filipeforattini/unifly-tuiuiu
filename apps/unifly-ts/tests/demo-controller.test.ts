import { describe, expect, it } from 'vitest';

import { DemoController } from '../src/runtime/demo-controller.js';
import { createDemoConfig } from '../src/services/demo-fixtures.js';

describe('DemoController', () => {
  it('connects and seeds a live snapshot', async () => {
    const controller = new DemoController(createDemoConfig());

    await controller.connect();

    const snapshot = controller.store.current();
    expect(snapshot.connectionState).toBe('connected');
    expect(snapshot.devices.length).toBeGreaterThan(0);
    expect(snapshot.metrics.throughputHistory.length).toBeGreaterThan(0);

    await controller.disconnect();
  });

  it('toggles demo pulse mode', () => {
    const controller = new DemoController(createDemoConfig());
    const before = controller.store.current().demoPulseEnabled;

    controller.toggleDemoPulse();

    expect(controller.store.current().demoPulseEnabled).toBe(!before);
  });
});
