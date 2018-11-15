import readline from 'readline';
import REPL from './REPL';
import {HEADER, USAGE} from "./constants";
import test from "./test";

let FLAGS = {
    debug: false,
    transpile: false
    test: false
};

console.log(HEADER);

const args = process.argv.slice(2);

for (const arg of args) {
    if (arg === '--test') {
        FLAGS.test = true;
    } else if (arg === '--debug') {
        FLAGS.debug = true;
    } else if (arg === '--transpile') {
        FLAGS.transpile = true
    }
}

if (FLAGS.test) {
    test({debug: FLAGS.debug, transpile: FLAGS.transpile});
    process.exit(0)
}

const rl = readline.createInterface({
    input: process.stdin,
    output: process.stdout
});

rl.on('line', line => {
    if (line === 'help') {
        console.log(USAGE);
    } else {
        if (!line.length) {
            console.log('Please, provide an expression!');
        }
        const run = REPL.run(line, {debug: FLAGS.debug, transpile: FLAGS.transpile});
        console.log('\x1b[32;1m=>\x1b[0m', run);
    }
    rl.prompt()
}).on('close', () => {
    console.log('\x1b[1;32mHave a great day!\x1b[0m');
    process.exit(0)
});

rl.prompt();