// Long arithmetic chain test
const x1 = 10;
const x2 = 20;
const x3 = 30;
const x4 = 40;
const x5 = 50;
const x6 = 60;
const x7 = 70;
const x8 = 80;

console.log(x1 + x2 + x3 + x4);
console.log(x5 + x6 + x7 + x8);
console.log(x1 * x2 + x3 * x4);
console.log(x5 * x6 + x7 * x8);
console.log(x1 + x2 * x3 - x4);
console.log(x5 - x6 * x7 + x8);
console.log(x1 * x2 * x3 / x4);
console.log(x5 * x6 / x7 / x8);

const chain1 = x1 + x2 + x3;
const chain2 = x4 + x5 + x6;
const chain3 = x7 + x8 + chain1;
const chain4 = chain2 + chain3 + x1;

console.log(chain1);
console.log(chain2);
console.log(chain3);
console.log(chain4);
