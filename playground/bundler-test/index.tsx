// DX Bundler Test - React-like TSX Application
import { useState } from './utils';
import { Component } from './components/Component';

interface AppProps {
  title: string;
  count?: number;
}

function App({ title, count = 0 }: AppProps) {
  const [value, setValue] = useState(count);
  
  const handleClick = () => {
    setValue(value + 1);
  };
  
  return (
    <div className="app">
      <h1>{title}</h1>
      <Component count={value} onClick={handleClick} />
      <button onClick={handleClick}>Increment</button>
    </div>
  );
}

export default App;
