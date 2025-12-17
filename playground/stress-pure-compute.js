// STRESS TEST: Pure computation heavy (no I/O until end)
const v1 = 100;
const v2 = 200;
const v3 = 300;
const v4 = 400;
const v5 = 500;
const v6 = 600;
const v7 = 700;
const v8 = 800;
const v9 = 900;
const v10 = 1000;

const r1 = Math.sqrt(v1);
const r2 = Math.sqrt(v2);
const r3 = Math.sqrt(v3);
const r4 = Math.sqrt(v4);
const r5 = Math.sqrt(v5);
const r6 = Math.sqrt(v6);
const r7 = Math.sqrt(v7);
const r8 = Math.sqrt(v8);
const r9 = Math.sqrt(v9);
const r10 = Math.sqrt(v10);

const s1 = r1 + r2;
const s2 = r3 + r4;
const s3 = r5 + r6;
const s4 = r7 + r8;
const s5 = r9 + r10;

const f1 = Math.floor(s1);
const f2 = Math.floor(s2);
const f3 = Math.floor(s3);
const f4 = Math.floor(s4);
const f5 = Math.floor(s5);

const total = f1 + f2 + f3 + f4 + f5;

console.log(total);
