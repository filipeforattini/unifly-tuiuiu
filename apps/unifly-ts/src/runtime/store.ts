import type { ControllerSnapshot } from '../domain/types.js';
import { ReactiveValue, type Unsubscribe } from './reactive-value.js';

export class DataStore {
  readonly #snapshot: ReactiveValue<ControllerSnapshot>;

  constructor(initialSnapshot: ControllerSnapshot) {
    this.#snapshot = new ReactiveValue(initialSnapshot);
  }

  current(): ControllerSnapshot {
    return this.#snapshot.get();
  }

  set(snapshot: ControllerSnapshot): void {
    this.#snapshot.set(snapshot);
  }

  update(updater: (snapshot: ControllerSnapshot) => ControllerSnapshot): void {
    this.#snapshot.update(updater);
  }

  subscribe(subscriber: (snapshot: ControllerSnapshot) => void): Unsubscribe {
    return this.#snapshot.subscribe(subscriber);
  }
}
