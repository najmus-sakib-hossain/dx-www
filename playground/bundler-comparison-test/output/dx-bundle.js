(function(){
var __dx_modules={};
var __dx_cache={};
function __dx_define(id,factory){__dx_modules[id]=factory;}
function __dx_require(id){
if(__dx_cache[id])return __dx_cache[id].exports;
var module=__dx_cache[id]={exports:{}};
__dx_modules[id](module.exports,__dx_require,module);
return module.exports;
}
__dx_define(0,function(exports,require,module){const { Calculator, add, multiply, operations } = __dx_require(1); const calc = new Calculator(); const result = calc.compute(5, 10); console.log('Result:', result); console.log('Add:', add(3, 4)); console.log('Multiply:', multiply(6, 7)); console.log('Operations:', Object.keys(operations)); console.log('History:', calc.getHistory()); exports.calc = calc; exports.result = result;});
__dx_define(1,function(exports,require,module){ function add(a, b){ return a + b; } function multiply(x, y){ return x * y; } const PI = 3.14159; class Calculator { history = []; compute(a, b){ const result = add(multiply(a, b), PI); this.history.push(result); return result; } getHistory(){ return this.history; } } const operations = { add: { execute: (x, y) => x + y }, sub: { execute: (x, y) => x - y }, mul: { execute: (x, y) => x * y }, div: { execute: (x, y) => x / y }, }; exports.add = add; exports.multiply = multiply; exports.PI = PI; exports.Calculator = Calculator; exports.operations = operations;});
__dx_require(0);})();