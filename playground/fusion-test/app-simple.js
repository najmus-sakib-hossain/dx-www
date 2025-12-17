// Simple React usage test
import React from './react-large.js';
import { ReactDOM } from './react-large.js';

function Counter() {
  const [count, setCount] = React.useState(0);
  
  return React.createElement('div', null,
    React.createElement('h1', null, 'Count: ' + count),
    React.createElement('button', { onClick: () => setCount(count + 1) }, 'Increment')
  );
}

const root = ReactDOM.createRoot(document.getElementById('root'));
root.render(React.createElement(Counter));
