import REPL from "../REPL";

const cases = {
    '(+ 55 6)': 61,
    '(- 7 6)': 1,
    '(* 3 6)': 18,
    '(/ 8 4)': 2,
    '(% 7 4)': 3,
    '(^ 2 10)': 1024,
    '(+ (+ 9 9) (* 9 9))': 99,
    '(+ (/ 8 5) (* (- (% 5 4) 6) 7))': -33.4,
    '(/ 1 0)': Infinity,
    '(+ 0.2 3)': 3.2,
    '(+ "hello" " " "world")': 'hello world',
    '(+ "2 + 5 = " (+ 2 5))': '2 + 5 = 7',
    '(+ "true = " true)': 'true = true'
};

export default function test({debug, transpile}) {
    const length = Object.keys(cases).length;
    let index = 1;

    for (const [code, answer] of Object.entries(cases)) {
        const result = REPL.run(code, {debug, transpile});
        if (result === answer) {
            console.log(`\x1b[32mCASE ${index++} of ${length} PASSED: ${code} = ${result}\x1b[0m`)
        } else {
            console.log(`\x1b[31mCASE ${index++} of ${length} FAILED: ${code} = ${result} (${answer} expected)\x1b[0m`)
        }
    }
}