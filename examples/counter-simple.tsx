// Example: Simple Counter App
// This will be compiled to Micro runtime (338 bytes)

function Counter() {
    const [count, setCount] = useState(0);
    
    return (
        <div>
            <h1>Counter: {count}</h1>
            <button onClick={() => setCount(count + 1)}>
                Increment
            </button>
        </div>
    );
}

export default Counter;
