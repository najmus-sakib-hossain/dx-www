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
__dx_define(0,function(exports,require,module){function useState(initial){ let state = initial; const setState = (newValue) => { state = newValue; render(); }; return [state, setState]; } function useEffect(fn, deps) { fn(); } exports.helpers = { format: (value)=> `Count: ${value}`, parse: (str)=> parseInt(str, 10), };});
__dx_define(1,function(exports,require,module){ function Component({ count, onClick }) { return null; } function UnusedComponent() { return ; }});
__dx_define(2,function(exports,require,module){const { useState } = require('./utils'); const { Component } = require('./components/Component'); function App({ title, count = 0 }) { const [value, setValue] = useState(count); const handleClick = () => { setValue(value + 1); }; return null; } module.exports = App;});
__dx_require(0);})();