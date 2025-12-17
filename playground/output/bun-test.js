// bundler-test/simple.js
function hello() {
  console.log("Hello from module 0");
}
var simple_default = hello;
export {
  simple_default as default
};
