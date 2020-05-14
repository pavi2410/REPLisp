import $REPL from "../src/REPLisp.js";

const cases = {
    '(+ 55 6)': 61,
    '(- 7 6)': 1,
    '(* 3 6)': 18,
    '(/ 8 4)': 2,
    '(% 7 4)': 3,
    '(^ 2 10)': 1024,
    '(> 9 5)': true,
    '(>= 8 6)': true,
    '(< 7 3)': false,
    '(<= 6 4)': false,
    '(= 5 5)': true,
    '(not false)': true,
    '(or true false)': true,
    '(and true true)': true,
    '(if true then 1 else 0)': 1,
    '(+ (+ 9 9) (* 9 9))': 99,
    '(+ (/ 8 5) (* (- (% 5 4) 6) 7))': -33.4,
    '(/ 1 0)': Infinity,
    '(+ 0.2 3)': 3.2,
    '(+ -9 9)': 0,
    '(print "hello world")': undefined,
    '(print "2 + 5 = " (+ 2 5))': undefined
};

let index = 1;
const length = Object.keys(cases).length;
let score = 0;
for (const [code, answer] of Object.entries(cases)) {
    const t1 = process.hrtime()[1];
    const result = $REPL(code);
    const t2 = process.hrtime()[1];
    if (result === answer) {
        score++;
        console.log(`\x1b[32m✔ ${index++}/${length} @ ${Math.round((t2 - t1) / 1000) / 1000}ms:\n=> ${code}\n== ${result}\x1b[0m`)
    } else {
        console.log(`\x1b[31m❌ ${index++}/${length}:\n=> ${code}\n== ${result} (${answer} expected)\x1b[0m`)
    }
    console.log('~'.repeat(25));
}
console.log(`Test score is ${Math.round(score * 100 / length)}%`);