import { Calculator, add, multiply, operations } from './utils';

const calc = new Calculator();
const result = calc.compute(5, 10);

console.log('Result:', result);
console.log('Add:', add(3, 4));
console.log('Multiply:', multiply(6, 7));
console.log('Operations:', Object.keys(operations));
console.log('History:', calc.getHistory());

export { calc, result };
