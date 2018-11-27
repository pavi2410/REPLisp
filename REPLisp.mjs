#!/usr/bin/node --experimental-modules --no-warnings --title=REPLisp
import readline from 'readline';
import fs from "fs";
import REPL from './Core/REPL.mjs';
import test from "./Tests/test.mjs";

const VERSION = '1.1';

const HEADER = `
\x1b[93mREPLisp v${VERSION}\x1b[0m by \x1b[95m@pavi2410\x1b[0m
\x1b[94;4mhttps://github.com/pavi2410/REPLisp\x1b[0m
- Hit \x1b[37m Ctrl + C \x1b[0m to exit`.trimLeft();

const USAGE = `
Usage: REPLisp [options...] [file]

Options:
    --test          Run tests
    --debug         Add debug info
    --help          Show this help text`.trimLeft();

let FLAGS = {
    debug: false,
    test: false
};

const args = process.argv.slice(2);

for (const arg of args) {
    if (arg.endsWith('.rep')) {
        FLAGS.file = arg;
    } else if (arg === '--test') {
        FLAGS.test = true;
    } else if (arg === '--debug') {
        FLAGS.debug = true;
    } else if (arg === '--help') {
        console.log(USAGE);
        process.exit()
    }
}

if (FLAGS.file) {
    fs.readFile(FLAGS.file, (err, data) => {
        const result = REPL.run(data.toString(), FLAGS.debug);
        console.log(result);
        process.exit()
    });
} else if (FLAGS.test) {
    test(FLAGS.debug);
    process.exit()
} else {
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
}
