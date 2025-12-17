// Comprehensive Runtime Test Suite - JavaScript & TypeScript
// Tests arithmetic, array operations, string manipulation, async, and more

// Test 1: Basic Arithmetic
function testArithmetic() {
  let sum = 0;
  for (let i = 0; i < 100000; i++) {
    sum += i * 2 - (i / 2) + (i % 3);
  }
  return sum;
}

// Test 2: Array Operations
function testArrayOps() {
  const arr = Array.from({ length: 10000 }, (_, i) => i);
  return arr
    .filter(x => x % 2 === 0)
    .map(x => x * 2)
    .reduce((acc, x) => acc + x, 0);
}

// Test 3: String Manipulation
function testStrings() {
  let result = "";
  for (let i = 0; i < 5000; i++) {
    result += `Item ${i}: ${i * 2}\n`;
  }
  return result.length;
}

// Test 4: Object Creation
function testObjects() {
  const objects = [];
  for (let i = 0; i < 10000; i++) {
    objects.push({
      id: i,
      name: `User ${i}`,
      score: i * 1.5,
      active: i % 2 === 0
    });
  }
  return objects.filter(o => o.active).length;
}

// Test 5: Recursive Fibonacci
function fib(n) {
  if (n <= 1) return n;
  return fib(n - 1) + fib(n - 2);
}

function testRecursion() {
  return fib(20);
}

// Test 6: Promise Chain
async function testAsync() {
  let sum = 0;
  for (let i = 0; i < 100; i++) {
    sum += await Promise.resolve(i);
  }
  return sum;
}

// Test 7: JSON Operations
function testJSON() {
  const data = {
    users: Array.from({ length: 1000 }, (_, i) => ({
      id: i,
      name: `User ${i}`,
      email: `user${i}@example.com`
    }))
  };
  const json = JSON.stringify(data);
  const parsed = JSON.parse(json);
  return parsed.users.length;
}

// Run all tests
async function runAll() {
  console.log('Test 1 (Arithmetic):', testArithmetic());
  console.log('Test 2 (Array Ops):', testArrayOps());
  console.log('Test 3 (Strings):', testStrings());
  console.log('Test 4 (Objects):', testObjects());
  console.log('Test 5 (Recursion):', testRecursion());
  console.log('Test 6 (Async):', await testAsync());
  console.log('Test 7 (JSON):', testJSON());
}

runAll();
