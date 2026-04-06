import { loadAppConfig } from './config.js';
import { DemoController } from './runtime/demo-controller.js';
import { RealController } from './runtime/real-controller.js';
import { createDemoConfig } from './services/demo-fixtures.js';
import { launchApp } from './ui/app.js';

const appConfig = loadAppConfig();
let controller;

if (appConfig.mode === 'real') {
  controller = new RealController(appConfig.controller);
} else {
  controller = new DemoController(createDemoConfig());
}

try {
  await launchApp(controller);
} catch (error) {
  if (appConfig.mode === 'real') {
    const fallback = new DemoController(createDemoConfig());
    console.error(
      `[unifly-ts] real mode bootstrap failed, falling back to demo: ${
        error instanceof Error ? error.message : String(error)
      }`,
    );
    await launchApp(fallback);
  } else {
    throw error;
  }
}
