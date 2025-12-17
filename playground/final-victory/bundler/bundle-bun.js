// mock-react.js
var React = {
  createElement: function(type, props, ...children) {
    return { type, props: props || {}, children };
  },
  useState: function(initial) {
    let state = initial;
    const setState = (newValue) => {
      state = typeof newValue === "function" ? newValue(state) : newValue;
    };
    return [state, setState];
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
  Component: class Component {
    constructor(props) {
      this.props = props;
    }
  }
};
var mock_react_default = React;

// app.js
function Button({ onClick, children }) {
  return mock_react_default.createElement("button", { onClick }, children);
}
function Counter() {
  const [count, setCount] = mock_react_default.useState(0);
  return mock_react_default.createElement("div", { className: "counter" }, mock_react_default.createElement("h1", null, `Count: ${count}`), mock_react_default.createElement(Button, {
    onClick: () => setCount(count + 1)
  }, "Increment"));
}
function TodoItem({ todo, onToggle }) {
  return mock_react_default.createElement("li", {
    onClick: () => onToggle(todo.id),
    style: { textDecoration: todo.done ? "line-through" : "none" }
  }, todo.text);
}
function TodoList() {
  const [todos, setTodos] = mock_react_default.useState([
    { id: 1, text: "Learn Dx", done: false },
    { id: 2, text: "Build app", done: false },
    { id: 3, text: "Deploy", done: false }
  ]);
  const toggle = (id) => {
    setTodos(todos.map((t) => t.id === id ? { ...t, done: !t.done } : t));
  };
  return mock_react_default.createElement("div", null, mock_react_default.createElement("h2", null, "Todo List"), mock_react_default.createElement("ul", null, ...todos.map((todo) => mock_react_default.createElement(TodoItem, { key: todo.id, todo, onToggle: toggle }))));
}
function App() {
  return mock_react_default.createElement("div", { className: "app" }, mock_react_default.createElement("header", null, mock_react_default.createElement("h1", null, "My Application")), mock_react_default.createElement(Counter), mock_react_default.createElement(TodoList));
}
var app_default = App;
export {
  app_default as default
};
