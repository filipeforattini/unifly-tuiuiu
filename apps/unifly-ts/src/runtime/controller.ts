import type { ControllerConfig, ControllerSnapshot } from '../domain/types.js';
import { DataStore } from './store.js';

export interface Controller {
  readonly config: ControllerConfig;
  readonly store: DataStore;
  connect(): Promise<void>;
  disconnect(): Promise<void>;
  refresh(): Promise<void>;
  toggleDemoPulse(): void;
}
