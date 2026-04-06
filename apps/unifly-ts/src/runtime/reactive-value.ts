export type Unsubscribe = () => void;
export type Subscriber<T> = (value: T) => void;

export class ReactiveValue<T> {
  readonly #subscribers = new Set<Subscriber<T>>();

  constructor(private value: T) {}

  get(): T {
    return this.value;
  }

  set(value: T): void {
    this.value = value;
    for (const subscriber of this.#subscribers) {
      subscriber(this.value);
    }
  }

  update(updater: (current: T) => T): void {
    this.set(updater(this.value));
  }

  subscribe(subscriber: Subscriber<T>): Unsubscribe {
    this.#subscribers.add(subscriber);
    subscriber(this.value);
    return () => {
      this.#subscribers.delete(subscriber);
    };
  }
}
