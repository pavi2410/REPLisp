import readline from 'readline';
import REPL from './REPL';
import {HEADER, USAGE} from "./constants";

import test from './test'

const rl = readline.createInterface({
    input: process.stdin,
    output: process.stdout
});

console.log(HEADER);
rl.prompt();

rl.on('line', line => {
    const [cmd, ...args] = line.split(' ');
    switch (cmd) {
        case 'run':
            if (!args.length) {
                console.log('Please, provide an expression!');
                break
            }
            console.log('\x1b[32;1m=>\x1b[0m', REPL.run(args.join(' ')));
            break;
        case 'test':
            test();
            break;
        case 'help':
            console.log(USAGE);
            break;
        case 'bye':
            rl.close();
            break;
        default:
            console.log(`Say what? I might have heard '${line}'`);
            break
    }
    rl.prompt()
}).on('close', () => {
    console.log('Have a great day!');
    process.exit(0)
});