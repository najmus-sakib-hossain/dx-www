 const React={createElement: function(tag,props,...children){return{tag,props,children};},useState: function(initial){return [initial,function(v){}];},useEffect: function(fn,deps){fn();},Component: function(){}};const ReactDOM={render: function(element,container){container.innerHTML=JSON.stringify(element);}};export{React,ReactDOM};export default React;
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
