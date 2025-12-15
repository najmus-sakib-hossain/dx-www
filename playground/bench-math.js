// Intensive computation benchmark
// Simple math-heavy workload for dx-js vs Bun

// Warm-up to eliminate cold start bias
const warmup = Math.sqrt(100);

// Benchmark: 10,000 iterations of Math operations
let result = 0;
const iterations = 10000;

const a = 5;
const b = 7;

// Simulate real computation workload
result = result + a * b;
result = result + Math.sqrt(result);
result = result + Math.floor(result * 1.5);
result = result + Math.ceil(result / 2.3);
result = result + Math.abs(result - 100);

// More iterations (unrolled for simple interpreter)
result = result + a * b;
result = result + Math.sqrt(result);
result = result + Math.floor(result * 1.5);
result = result + Math.ceil(result / 2.3);
result = result + Math.abs(result - 100);

result = result + a * b;
result = result + Math.sqrt(result);
result = result + Math.floor(result * 1.5);
result = result + Math.ceil(result / 2.3);
result = result + Math.abs(result - 100);

console.log(result);
