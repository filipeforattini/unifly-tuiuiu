import { describe, expect, it } from 'vitest';

import { entityId, normalizeMac } from '../src/domain/identity.js';

describe('identity helpers', () => {
  it('detects uuid entity ids', () => {
    expect(entityId('550e8400-e29b-41d4-a716-446655440000')).toEqual({
      kind: 'uuid',
      value: '550e8400-e29b-41d4-a716-446655440000',
    });
  });

  it('keeps legacy ids as legacy', () => {
    expect(entityId('507f1f77bcf86cd799439011')).toEqual({
      kind: 'legacy',
      value: '507f1f77bcf86cd799439011',
    });
  });

  it('normalizes common mac formats', () => {
    expect(normalizeMac('AA-BB-CC-DD-EE-FF')).toBe('aa:bb:cc:dd:ee:ff');
  });
});
