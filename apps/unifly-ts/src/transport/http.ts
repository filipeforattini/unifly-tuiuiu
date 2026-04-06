export class HttpError extends Error {
  constructor(
    message: string,
    readonly status: number,
    readonly body: string,
  ) {
    super(message);
    this.name = 'HttpError';
  }
}

export interface JsonHttpClientOptions {
  baseUrl: string;
  defaultHeaders?: Record<string, string>;
}

export class JsonHttpClient {
  readonly #baseUrl: URL;
  readonly #defaultHeaders: Record<string, string>;

  constructor(options: JsonHttpClientOptions) {
    this.#baseUrl = new URL(options.baseUrl);
    this.#defaultHeaders = {
      accept: 'application/json',
      ...options.defaultHeaders,
    };
  }

  async get<T>(path: string, query?: Record<string, string | number | undefined>): Promise<T> {
    const url = new URL(path, this.#baseUrl);
    if (query) {
      for (const [key, value] of Object.entries(query)) {
        if (value !== undefined) {
          url.searchParams.set(key, String(value));
        }
      }
    }

    const response = await fetch(url, {
      method: 'GET',
      headers: this.#defaultHeaders,
    });

    const text = await response.text();
    if (!response.ok) {
      throw new HttpError(`GET ${url.pathname} failed with ${response.status}`, response.status, text);
    }

    return JSON.parse(text) as T;
  }
}
