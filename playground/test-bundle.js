const fs = require('fs');
const code = fs.readFileSync('playground/output/simple-bundle.js', 'utf8');
console.log('File length:', code.length);
console.log('Last 50 chars:', code.slice(-50));

try {
    eval(code);
    console.log('✅ Bundle executed successfully!');
} catch (e) {
    console.log('❌ Error:', e.message);
    console.log('Stack:', e.stack);
}
