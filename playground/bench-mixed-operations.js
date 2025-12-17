// Mixed operations: Math + Arithmetic + Comparisons
const val1 = 144;
const val2 = 25;
const val3 = 16;

const sqrt1 = Math.sqrt(val1);
const sqrt2 = Math.sqrt(val2);
const sqrt3 = Math.sqrt(val3);

console.log(sqrt1 > sqrt2);
console.log(sqrt2 > sqrt3);
console.log(sqrt1 + sqrt2);
console.log(sqrt2 * sqrt3);

const sum = sqrt1 + sqrt2 + sqrt3;
const prod = sqrt1 * sqrt2 * sqrt3;

console.log(sum);
console.log(prod);
console.log(sum > prod);
console.log(prod > sum);

const floor_sum = Math.floor(sum);
const ceil_prod = Math.ceil(prod);

console.log(floor_sum);
console.log(ceil_prod);
console.log(floor_sum < ceil_prod);
console.log(ceil_prod > floor_sum);
