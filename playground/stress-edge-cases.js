// STRESS TEST: Edge cases and boundary conditions

// Zero values
const zero = 0;
console.log(zero);
console.log(Math.sqrt(zero));
console.log(Math.floor(zero));
console.log(Math.ceil(zero));

// Large numbers
const large = 999999999;
console.log(large);
console.log(Math.sqrt(large));

// Negative results (via subtraction)
const neg = 10 - 50;
console.log(neg);
console.log(Math.abs(neg));

// Float precision
const float1 = 10 / 3;
const float2 = 100 / 7;
console.log(float1);
console.log(float2);
console.log(Math.floor(float1));
console.log(Math.ceil(float2));

// Boolean values
const t = true;
const f = false;
console.log(t);
console.log(f);

// Comparison edge cases
const eq1 = 10 < 10;
const eq2 = 10 > 10;
console.log(eq1);
console.log(eq2);

// Many variables at once (test 32-slot limit)
const a1 = 1;
const a2 = 2;
const a3 = 3;
const a4 = 4;
const a5 = 5;
const a6 = 6;
const a7 = 7;
const a8 = 8;
const a9 = 9;
const a10 = 10;
const a11 = 11;
const a12 = 12;
const a13 = 13;
const a14 = 14;
const a15 = 15;
const a16 = 16;
const a17 = 17;
const a18 = 18;
const a19 = 19;
const a20 = 20;
const a21 = 21;
const a22 = 22;
const a23 = 23;
const a24 = 24;
const a25 = 25;
const a26 = 26;
const a27 = 27;
const a28 = 28;
const a29 = 29;
const a30 = 30;
const a31 = 31;
const a32 = 32;

const sum = a1 + a32;
console.log(sum);
