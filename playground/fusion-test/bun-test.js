// react-large.js
var React = {
  createElement: function(type, props, ...children) {
    return { type, props: props || {}, children };
  },
  useState: function(initial) {
    return [initial, function(newValue) {}];
  },
  useEffect: function(fn, deps) {
    fn();
  },
  useCallback: function(fn, deps) {
    return fn;
  },
  useMemo: function(fn, deps) {
    return fn();
  },
  useRef: function(initial) {
    return { current: initial };
  },
  Component: function() {},
  PureComponent: function() {},
  Fragment: Symbol("Fragment"),
  StrictMode: Symbol("StrictMode"),
  Suspense: Symbol("Suspense"),
  lazy: function(factory) {
    return factory;
  },
  memo: function(component) {
    return component;
  },
  forwardRef: function(render) {
    return render;
  },
  createContext: function(defaultValue) {
    return { Provider: {}, Consumer: {}, _currentValue: defaultValue };
  },
  createRef: function() {
    return { current: null };
  }
};
var ReactDOM = {
  render: function(element, container) {
    container.innerHTML = JSON.stringify(element);
  },
  hydrate: function(element, container) {
    container.innerHTML = JSON.stringify(element);
  },
  createPortal: function(children, container) {
    return { children, container };
  },
  createRoot: function(container) {
    return { render: function(element) {
      container.innerHTML = JSON.stringify(element);
    } };
  }
};
var react_large_default = React;

// app-simple.js
function Counter() {
  const [count, setCount] = react_large_default.useState(0);
  return react_large_default.createElement("div", null, react_large_default.createElement("h1", null, "Count: " + count), react_large_default.createElement("button", { onClick: () => setCount(count + 1) }, "Increment"));
}
var root = ReactDOM.createRoot(document.getElementById("root"));
root.render(react_large_default.createElement(Counter));
