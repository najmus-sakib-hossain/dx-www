// Simple TypeScript modules for benchmarking (no external dependencies)

export function add(a: number, b: number): number {
  return a + b;
}

export function multiply(x: number, y: number): number {
  return x * y;
}

export const PI: number = 3.14159;

export class Calculator {
  private history: number[] = [];
  
  compute(a: number, b: number): number {
    const result = add(multiply(a, b), PI);
    this.history.push(result);
    return result;
  }
  
  getHistory(): number[] {
    return this.history;
  }
}

export interface MathOp {
  execute(x: number, y: number): number;
}

export const operations: Record<string, MathOp> = {
  add: { execute: (x, y) => x + y },
  sub: { execute: (x, y) => x - y },
  mul: { execute: (x, y) => x * y },
  div: { execute: (x, y) => x / y },
};
