import { loadAppConfig } from './config.js';
import { DemoController } from './runtime/demo-controller.js';
import type { Controller } from './runtime/controller.js';
import { RealController } from './runtime/real-controller.js';
import { createDemoConfig } from './services/demo-fixtures.js';
import { launchApp } from './ui/app.js';

const appConfig = loadAppConfig();
let controller: Controller;

if (appConfig.mode === 'real') {
  controller = new RealController(appConfig.controller);
} else {
  controller = new DemoController(appConfig.controller);
}

try {
  await launchApp(controller);
} catch (error) {
  if (appConfig.mode === 'real') {
    const fallbackConfig = createDemoConfig();
    fallbackConfig.controllerUrl = appConfig.controller.controllerUrl;
    fallbackConfig.site = appConfig.controller.site;
    const fallback = new DemoController(fallbackConfig);
    fallback.store.update((snapshot) => ({
      ...snapshot,
      runtime: {
        appMode: 'real',
        dataSource: 'fallback-demo',
        controllerUrl: appConfig.controller.controllerUrl,
        site: appConfig.controller.site,
        statusMessage: 'real bootstrap failed; running fallback demo',
        lastError: error instanceof Error ? error.message : String(error),
      },
    }));
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
