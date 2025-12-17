// Entry point
import { greet, VERSION } from './module-a.js';
import { farewell } from './module-b.js';

console.log(greet('World'));
console.log('Version:', VERSION);
console.log(farewell('World'));
