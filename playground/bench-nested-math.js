// Nested Math operations test
const a = 100;
const b = 25;
const c = 16;

console.log(Math.sqrt(Math.sqrt(a)));
console.log(Math.floor(Math.sqrt(b) + 10));
console.log(Math.ceil(Math.sqrt(c) * 2));
console.log(Math.abs(Math.floor(-42.7)));
console.log(Math.round(Math.sqrt(50)));

const nested1 = Math.sqrt(a + b);
const nested2 = Math.floor(nested1 * 2);
const nested3 = Math.ceil(nested2 / 3);

console.log(nested1);
console.log(nested2);
console.log(nested3);

const complex = Math.sqrt(Math.floor(a / 2) + Math.ceil(b * 1.5));
console.log(complex);
