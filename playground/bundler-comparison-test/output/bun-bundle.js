// ../benchmark-simple/utils.ts
function add(a, b) {
  return a + b;
}
function multiply(x, y) {
  return x * y;
}
var PI = 3.14159;

class Calculator {
  history = [];
  compute(a, b) {
    const result = add(multiply(a, b), PI);
    this.history.push(result);
    return result;
  }
  getHistory() {
    return this.history;
  }
}
var operations = {
  add: { execute: (x, y) => x + y },
  sub: { execute: (x, y) => x - y },
  mul: { execute: (x, y) => x * y },
  div: { execute: (x, y) => x / y }
};

// ../benchmark-simple/index.ts
var calc = new Calculator;
var result = calc.compute(5, 10);
console.log("Result:", result);
console.log("Add:", add(3, 4));
console.log("Multiply:", multiply(6, 7));
console.log("Operations:", Object.keys(operations));
console.log("History:", calc.getHistory());
export {
  result,
  calc
};
