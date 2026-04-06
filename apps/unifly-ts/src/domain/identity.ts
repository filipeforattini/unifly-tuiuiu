import type { EntityId } from './types.js';

const UUID_RE =
  /^[0-9a-f]{8}-[0-9a-f]{4}-[1-5][0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$/i;

export function entityId(value: string): EntityId {
  return UUID_RE.test(value)
    ? { kind: 'uuid', value: value.toLowerCase() }
    : { kind: 'legacy', value };
}

export function normalizeMac(raw: string): string {
  const compact = raw.replaceAll(':', '').replaceAll('-', '').trim().toLowerCase();
  if (compact.length !== 12 || /[^0-9a-f]/.test(compact)) {
    return raw.trim().toLowerCase();
  }

  const pairs: string[] = [];
  for (let index = 0; index < compact.length; index += 2) {
    pairs.push(compact.slice(index, index + 2));
  }

  return pairs.join(':');
}
