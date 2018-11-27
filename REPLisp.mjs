import readline from 'readline';
import REPL from './Core/REPL.mjs';
import {HEADER, USAGE} from "./CONSTANTS.mjs";
import test from "./Tests/test.mjs";
import Lexer from "./Core/Lexer.mjs";
import fs from "fs";
import Parser from "./Core/Parser.mjs";

let FLAGS = {
    debug: false,
    test: false,
    file: false
};

const args = process.argv.slice(2);

for (const arg of args) {
    switch (arg) {
        case '--file':
            FLAGS.file = true;
            break;
        case '--test':
            FLAGS.test = true;
            break;
        case '--debug':
            FLAGS.debug = true;
            break;
    }
}

if (FLAGS.file) {
    fs.readFile('./hello.rsp', (err, data) => {
        const tokens = Lexer.tokenize(data.toString());
        const ast = Parser.parse(tokens);
        console.log(tokens, ast);

        process.exit()
    });
} else if (FLAGS.test) {
    test(FLAGS.debug);
    process.exit()
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
        const run = REPL.run(line, FLAGS.debug);
        console.log('\x1b[1;37m' + run + '\x1b[0m');
    }
    rl.prompt()
}).on('close', () => {
    console.log('\x1b[1;32mHave a great day!\x1b[0m');
    process.exit()
});

console.log(HEADER);
rl.prompt();