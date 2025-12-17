// Benchmark: Compare dx-js-runtime vs Bun
// ==========================================
//
// Run with dx-js-runtime:
//   cargo run -p dx-js-runtime --release -- playground/test.ts
//
// Run with Bun:
//   bun run playground/test.ts
//
// Note: Install Bun first if not installed:
//   powershell -c "irm bun.sh/install.ps1 | iex"
//   or: npm install -g bun

// ─────────────────────────────────────────────────────────────
// BENCHMARK 1: Simple Arithmetic
// ─────────────────────────────────────────────────────────────

function add(a: number, b: number): number {
    return a + b;
}

function multiply(a: number, b: number): number {
    return a * b;
}

// ─────────────────────────────────────────────────────────────
// BENCHMARK 2: Fibonacci (Recursive)
// ─────────────────────────────────────────────────────────────

function fib(n: number): number {
    if (n <= 1) return n;
    return fib(n - 1) + fib(n - 2);
}

// ─────────────────────────────────────────────────────────────
// BENCHMARK 3: Array Operations
// ─────────────────────────────────────────────────────────────

function sumArray(arr: number[]): number {
    let sum = 0;
    for (let i = 0; i < arr.length; i++) {
        sum += arr[i];
    }
    return sum;
}

function mapDouble(arr: number[]): number[] {
    return arr.map(x => x * 2);
}

function filterEven(arr: number[]): number[] {
    return arr.filter(x => x % 2 === 0);
}

// ─────────────────────────────────────────────────────────────
// BENCHMARK 4: Object Manipulation
// ─────────────────────────────────────────────────────────────

interface User {
    id: number;
    name: string;
    score: number;
    active: boolean;
}

function createUsers(count: number): User[] {
    const users: User[] = [];
    for (let i = 0; i < count; i++) {
        users.push({
            id: i,
            name: `User ${i}`,
            score: Math.floor(Math.random() * 100),
            active: i % 2 === 0
        });
    }
    return users;
}

function processUsers(users: User[]): number {
    return users
        .filter(u => u.score > 50 && u.active)
        .map(u => u.score * 2)
        .reduce((a, b) => a + b, 0);
}

// ─────────────────────────────────────────────────────────────
// BENCHMARK 5: String Operations
// ─────────────────────────────────────────────────────────────

function concatStrings(count: number): string {
    let result = "";
    for (let i = 0; i < count; i++) {
        result += `item-${i}-`;
    }
    return result;
}

// ─────────────────────────────────────────────────────────────
// RUN BENCHMARKS
// ─────────────────────────────────────────────────────────────

const ITERATIONS = 1000;

console.log("╔══════════════════════════════════════════════════════════════╗");
console.log("║           dx-js-runtime vs Bun Benchmark                      ║");
console.log("╚══════════════════════════════════════════════════════════════╝");
console.log("");

// Benchmark 1: Fibonacci
console.log("─── Benchmark 1: Fibonacci(25) ───");
const fibStart = Date.now();
let fibResult = 0;
for (let i = 0; i < ITERATIONS; i++) {
    fibResult = fib(25);
}
const fibTime = Date.now() - fibStart;
console.log(`  Result: ${fibResult}`);
console.log(`  Time: ${fibTime}ms for ${ITERATIONS} iterations`);
console.log(`  Ops/sec: ${Math.round(ITERATIONS / (fibTime / 1000))}`);
console.log("");

// Benchmark 2: Array Sum (10k elements)
console.log("─── Benchmark 2: Array Sum (10k elements) ───");
const arr = Array.from({ length: 10000 }, (_, i) => i);
const arrStart = Date.now();
let arrResult = 0;
for (let i = 0; i < ITERATIONS; i++) {
    arrResult = sumArray(arr);
}
const arrTime = Date.now() - arrStart;
console.log(`  Result: ${arrResult}`);
console.log(`  Time: ${arrTime}ms for ${ITERATIONS} iterations`);
console.log(`  Ops/sec: ${Math.round(ITERATIONS / (arrTime / 1000))}`);
console.log("");

// Benchmark 3: Map + Filter (10k elements)
console.log("─── Benchmark 3: Map + Filter (10k elements) ───");
const mapStart = Date.now();
let mapResult: number[] = [];
for (let i = 0; i < ITERATIONS; i++) {
    mapResult = filterEven(mapDouble(arr));
}
const mapTime = Date.now() - mapStart;
console.log(`  Result length: ${mapResult.length}`);
console.log(`  Time: ${mapTime}ms for ${ITERATIONS} iterations`);
console.log(`  Ops/sec: ${Math.round(ITERATIONS / (mapTime / 1000))}`);
console.log("");

// Benchmark 4: Object Processing (1k users)
console.log("─── Benchmark 4: Object Processing (1k users) ───");
const users = createUsers(1000);
const objStart = Date.now();
let objResult = 0;
for (let i = 0; i < ITERATIONS; i++) {
    objResult = processUsers(users);
}
const objTime = Date.now() - objStart;
console.log(`  Result: ${objResult}`);
console.log(`  Time: ${objTime}ms for ${ITERATIONS} iterations`);
console.log(`  Ops/sec: ${Math.round(ITERATIONS / (objTime / 1000))}`);
console.log("");

// Benchmark 5: String Concatenation
console.log("─── Benchmark 5: String Concat (1k items) ───");
const strStart = Date.now();
let strResult = "";
for (let i = 0; i < ITERATIONS; i++) {
    strResult = concatStrings(100);
}
const strTime = Date.now() - strStart;
console.log(`  Result length: ${strResult.length}`);
console.log(`  Time: ${strTime}ms for ${ITERATIONS} iterations`);
console.log(`  Ops/sec: ${Math.round(ITERATIONS / (strTime / 1000))}`);
console.log("");

// Summary
console.log("╔══════════════════════════════════════════════════════════════╗");
console.log("║                         SUMMARY                              ║");
console.log("╠══════════════════════════════════════════════════════════════╣");
console.log(`║  Fibonacci:        ${fibTime.toString().padStart(6)}ms                              ║`);
console.log(`║  Array Sum:        ${arrTime.toString().padStart(6)}ms                              ║`);
console.log(`║  Map + Filter:     ${mapTime.toString().padStart(6)}ms                              ║`);
console.log(`║  Object Process:   ${objTime.toString().padStart(6)}ms                              ║`);
console.log(`║  String Concat:    ${strTime.toString().padStart(6)}ms                              ║`);
console.log("╠══════════════════════════════════════════════════════════════╣");
const totalTime = fibTime + arrTime + mapTime + objTime + strTime;
console.log(`║  TOTAL:           ${totalTime.toString().padStart(6)}ms                              ║`);
console.log("╚══════════════════════════════════════════════════════════════╝");
