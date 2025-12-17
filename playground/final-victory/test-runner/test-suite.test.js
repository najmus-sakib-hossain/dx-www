// Test Suite for Test Runner
import { describe, it, expect } from 'dx-test';

describe('Math Operations', () => {
  it('should add numbers correctly', () => {
    expect(2 + 2).toBe(4);
    expect(10 + 5).toBe(15);
  });

  it('should subtract numbers correctly', () => {
    expect(10 - 5).toBe(5);
    expect(100 - 50).toBe(50);
  });

  it('should multiply numbers correctly', () => {
    expect(5 * 5).toBe(25);
    expect(12 * 3).toBe(36);
  });

  it('should divide numbers correctly', () => {
    expect(10 / 2).toBe(5);
    expect(100 / 4).toBe(25);
  });
});

describe('Array Operations', () => {
  it('should filter arrays', () => {
    const arr = [1, 2, 3, 4, 5];
    const even = arr.filter(x => x % 2 === 0);
    expect(even.length).toBe(2);
    expect(even[0]).toBe(2);
  });

  it('should map arrays', () => {
    const arr = [1, 2, 3];
    const doubled = arr.map(x => x * 2);
    expect(doubled[0]).toBe(2);
    expect(doubled[1]).toBe(4);
    expect(doubled[2]).toBe(6);
  });

  it('should reduce arrays', () => {
    const arr = [1, 2, 3, 4];
    const sum = arr.reduce((acc, x) => acc + x, 0);
    expect(sum).toBe(10);
  });
});

describe('String Operations', () => {
  it('should concatenate strings', () => {
    expect('hello' + ' ' + 'world').toBe('hello world');
  });

  it('should get string length', () => {
    expect('test'.length).toBe(4);
  });

  it('should transform strings', () => {
    expect('HELLO'.toLowerCase()).toBe('hello');
    expect('world'.toUpperCase()).toBe('WORLD');
  });
});

describe('Object Operations', () => {
  it('should create objects', () => {
    const obj = { name: 'John', age: 30 };
    expect(obj.name).toBe('John');
    expect(obj.age).toBe(30);
  });

  it('should check object keys', () => {
    const obj = { a: 1, b: 2, c: 3 };
    const keys = Object.keys(obj);
    expect(keys.length).toBe(3);
  });
});

describe('Async Operations', () => {
  it('should handle promises', async () => {
    const result = await Promise.resolve(42);
    expect(result).toBe(42);
  });

  it('should handle async functions', async () => {
    const getData = async () => {
      return 'data';
    };
    const result = await getData();
    expect(result).toBe('data');
  });
});
