import { describe, it, expect } from 'vitest';
import { EntityType, RelationType } from './index.js';

describe('EntityType', () => {
  it('has exactly 18 values', () => {
    expect(Object.values(EntityType)).toHaveLength(18);
  });

  it('GuidanceEpic equals guidance_epic', () => {
    expect(EntityType.GuidanceEpic).toBe('guidance_epic');
  });
});

describe('RelationType', () => {
  it('has exactly 8 values', () => {
    expect(Object.values(RelationType)).toHaveLength(8);
  });

  it('RelatedTo equals related_to', () => {
    expect(RelationType.RelatedTo).toBe('related_to');
  });
});
