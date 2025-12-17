(function(){
var m={};
var c={};
function def(id,fn){m[id]=fn;}
function req(id){if(c[id])return c[id].exports;var mod=c[id]={exports:{}};m[id](mod.exports,req,mod);return mod.exports;}
def(0,function(exports,require,module){module.exports={x:1};});
req(0);
})();
