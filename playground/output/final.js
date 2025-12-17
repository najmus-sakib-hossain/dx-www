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
__dx_define(0,function(exports,require,module){const { Calculator, add, multiply, operations } = require('./utils'); const calc = new Calculator(); const result = calc.compute(5, 10); console.log('Result:', result); console.log('Add:', add(3, 4)); console.log('Multiply:', multiply(6, 7)); console.log('Operations:', Object.keys(operations)); console.log('History:', calc.getHistory()); export { calc, result };});
__dx_define(1,function(exports,require,module){ function add(a, b){ return a + b; } function multiply(x, y){ return x * y; } exports.PI: number = 3.14159; export class Calculator { private history: number[] = []; compute(a: number, b){ const result = add(multiply(a, b), PI); this.history.push(result); return result; } getHistory(){ return this.history; } } export exports.operations: Record});
__dx_require(0);})();