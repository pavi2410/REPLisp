const REPLisp = require('./REPLisp.js');

const cases = {
    '(+ 55 6)': 61,
    '(- 7 6)': 1,
    '(* 3 6)': 18,
    '(/ 8 4)': 2,
    '(% 7 4)': 3,
    '(^ 2 10)': 1024,
    '(! 5)': 120,
    '(> 9 5)': true,
    '(>= 8 6)': true,
    '(< 7 3)': false,
    '(<= 6 4)': false,
    '(== 5 5)': true,
    '(not false)': true,
    '(or true false)': true,
    '(and true true)': true,
    '(if true then 1 else 0)': 1,
    '(+ (+ 9 9) (* 9 9))': 99,
    '(+ (/ 8 5) (* (- (% 5 4) 6) 7))': -33.4,
    '(/ 1 0)': Infinity,
    '(+ 0.2 3)': 3.2,
    '(+ -9 9)': 0,
    '(+ "hello" " " "world")': 'hello world',
    '(+ "2 + 5 = " (+ 2 5))': '2 + 5 = 7',
    '(+ "true = " true)': 'true = true'
};

const length = Object.keys(cases).length;
let index = 1;

for (const [code, answer] of Object.entries(cases)) {
    const result = REPLisp.REPL(code);
    if (result === answer) {
        console.log(`\x1b[32mCASE ${index++} of ${length} PASSED: ${code} = ${result}\x1b[0m`)
    } else {
        console.log(`\x1b[31mCASE ${index++} of ${length} FAILED: ${code} = ${result} (${answer} expected)\x1b[0m`)
    }
}