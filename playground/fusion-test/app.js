// User application code
import { React, ReactDOM } from './react-mock.js';

function App() {
  const [count, setCount] = React.useState(0);
  return React.createElement('div', null, 
    React.createElement('h1', null, 'Count: ' + count),
    React.createElement('button', { onClick: () => setCount(count + 1) }, 'Increment')
  );
}

ReactDOM.render(React.createElement(App), document.getElementById('root'));
