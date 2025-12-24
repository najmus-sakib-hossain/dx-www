// Mock React library (simulates a pre-compiled react.js)
const React = {
  createElement: function(tag, props, ...children) {
    return { tag, props, children };
  },
  useState: function(initial) {
    return [initial, function(v) {}];
  },
  useEffect: function(fn, deps) {
    fn();
  },
  Component: function() {}
};
const ReactDOM = {
  render: function(element, container) {
    container.innerHTML = JSON.stringify(element);
  }
};
export { React, ReactDOM };
export default React;
