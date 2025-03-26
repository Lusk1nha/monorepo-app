import { add } from './mockup';

describe('Mockup add test', () => {
  it('should return 3', () => {
    expect(add(1, 2)).toBe(3);
  });
});
