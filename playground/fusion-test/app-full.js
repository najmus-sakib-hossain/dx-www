// Full React usage test - use all major React features
import React, { createElement, useState, useEffect, useCallback, useMemo, useRef, Fragment, lazy, memo } from './react-large.js';
import { ReactDOM } from './react-large.js';

// Force usage of all exported functions
const MemoizedButton = memo(function Button({ onClick, children }) {
  return createElement('button', { onClick }, children);
});

function Counter() {
  const [count, setCount] = useState(0);
  const [items, setItems] = useState([]);
  const countRef = useRef(count);
  
  const increment = useCallback(() => {
    setCount(c => c + 1);
  }, []);
  
  const doubled = useMemo(() => count * 2, [count]);
  
  useEffect(() => {
    countRef.current = count;
    setItems(prev => [...prev, count]);
  }, [count]);
  
  return createElement(Fragment, null,
    createElement('h1', null, 'Count: ' + count + ' (doubled: ' + doubled + ')'),
    createElement(MemoizedButton, { onClick: increment }, 'Increment'),
    createElement('ul', null, 
      items.map((item, i) => createElement('li', { key: i }, 'Item ' + item))
    )
  );
}

const App = lazy(() => Counter);
const root = ReactDOM.createRoot(document.getElementById('root'));
root.render(createElement(App));

// Force all internal functions to be included
const allInternals = [];
for (let i = 0; i < 500; i++) {
  const fn = eval('internal_fn_' + String(i).padStart(4, '0'));
  if (typeof fn === 'function') allInternals.push(fn(1, 2, 3));
}
console.log(allInternals.length);
