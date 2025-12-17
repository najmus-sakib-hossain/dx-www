// Bundler Test - React-style Application
import React from './mock-react.js';

function Button({ onClick, children }) {
  return React.createElement('button', { onClick }, children);
}

function Counter() {
  const [count, setCount] = React.useState(0);
  
  return React.createElement('div', { className: 'counter' },
    React.createElement('h1', null, `Count: ${count}`),
    React.createElement(Button, { 
      onClick: () => setCount(count + 1) 
    }, 'Increment')
  );
}

function TodoItem({ todo, onToggle }) {
  return React.createElement('li', { 
    onClick: () => onToggle(todo.id),
    style: { textDecoration: todo.done ? 'line-through' : 'none' }
  }, todo.text);
}

function TodoList() {
  const [todos, setTodos] = React.useState([
    { id: 1, text: 'Learn Dx', done: false },
    { id: 2, text: 'Build app', done: false },
    { id: 3, text: 'Deploy', done: false }
  ]);

  const toggle = (id) => {
    setTodos(todos.map(t => 
      t.id === id ? { ...t, done: !t.done } : t
    ));
  };

  return React.createElement('div', null,
    React.createElement('h2', null, 'Todo List'),
    React.createElement('ul', null,
      ...todos.map(todo => 
        React.createElement(TodoItem, { key: todo.id, todo, onToggle: toggle })
      )
    )
  );
}

function App() {
  return React.createElement('div', { className: 'app' },
    React.createElement('header', null,
      React.createElement('h1', null, 'My Application')
    ),
    React.createElement(Counter),
    React.createElement(TodoList)
  );
}

export default App;
