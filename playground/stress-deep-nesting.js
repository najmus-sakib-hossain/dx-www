// STRESS TEST: Deeply nested Math operations
const base = 1000000;

// 5 levels of nesting
const level1 = Math.sqrt(base);
const level2 = Math.sqrt(level1);
const level3 = Math.sqrt(level2);
const level4 = Math.sqrt(level3);
const level5 = Math.sqrt(level4);

console.log(level5);

// Complex nested arithmetic
const a = 10;
const b = 20;
const c = 30;
const d = 40;
const e = 50;

const nested1 = a + b * c - d / e;
const nested2 = Math.floor(nested1 * 2);
const nested3 = Math.ceil(nested2 / 3);
const nested4 = Math.sqrt(nested3 + 100);
const nested5 = Math.abs(nested4 - 50);
const nested6 = Math.round(nested5 * 1.5);

console.log(nested6);

// Multiple operations in single expression
const complex1 = Math.sqrt(Math.floor(a * b) + Math.ceil(c / d));
const complex2 = Math.abs(Math.round(complex1 - e));
const complex3 = Math.floor(Math.sqrt(complex2 * 2));

console.log(complex3);

// Chain of comparisons
const x = 100;
const y = 200;
const z = 150;

const result1 = x < y;
const result2 = y > z;
const result3 = z < x;
const final = result1;

console.log(final);
console.log(result2);
console.log(result3);
