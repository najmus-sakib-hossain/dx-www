// Sample JavaScript file to test Biome formatter and linter
// This file demonstrates various JavaScript features

function greet(name) {
    console.log("Hello, " + name + "!");
}

const calculateSum = (a, b) => {
  return a+b;
}

class Person {
  constructor(name, age) {
    this.name = name;
    this.age = age;
  }

  introduce() {
    console.log(`I am ${this.name} and I'm ${this.age} years old.`);
  }
}

// Array methods
const numbers = [1, 2, 3, 4, 5];
const doubled = numbers.map(n => n * 2);
const filtered = numbers.filter(n => n > 2);

// Async/await
async function fetchData(url) {
  try {
    const response = await fetch(url);
    const data = await response.json();
    return data;
  } catch (error) {
    console.error("Error fetching data:", error);
  }
}

// Destructuring
const user = { name: "John", email: "john@example.com", age: 30 };
const { name, email } = user;

// Template literals
const greeting = `Hello, ${name}!`;

// Export
export { greet, calculateSum, Person, fetchData };
