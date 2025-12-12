import { useState } from 'dx';

export default function App() {
  const [count, setCount] = useState(0);
  
  return (
    <div class="container">
      <h1>Welcome to demo-app!</h1>
      <div class="counter">
        <button onClick={() => setCount(count - 1)}>-</button>
        <span class="count">{count}</span>
        <button onClick={() => setCount(count + 1)}>+</button>
      </div>
      <p class="info">
        Edit <code>src/App.tsx</code> and save to reload.
      </p>
    </div>
  );
}
