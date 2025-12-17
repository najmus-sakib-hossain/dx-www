const fs = require('fs');
const code = fs.readFileSync('playground/output/simple-bundle.js', 'utf8');

let opens = 0, closes = 0;
for (let i = 0; i < code.length; i++) {
    if (code[i] === '(' || code[i] === '{') opens++;
    if (code[i] === ')' || code[i] === '}') closes++;
}

console.log('Opening ( and {:', opens);
console.log('Closing ) and }:', closes);
console.log('Match:', opens === closes ? '✅ YES' : '❌ NO');

// Show each line with counts
const lines = code.split('\n');
lines.forEach((line, i) => {
    let o = 0, c = 0;
    for (let ch of line) {
        if (ch === '(' || ch === '{') o++;
        if (ch === ')' || ch === '}') c++;
    }
    if (o > 0 || c > 0) {
        console.log(`Line ${i+1}: +${o} -${c} | ${line.substring(0, 80)}`);
    }
});
