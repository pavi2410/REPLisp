import {start} from "repl";
import {readFileSync} from "fs";

import esMain from 'es-main'

function clean(lines) {
    return lines
        .replace(/#.*/g, '') // remove comments
        .replace(/\s+/g, ' '); // remove any whitespace
}

function tokenize(lines) {
    return lines
        .replace(/"[^"]*?"/g, s => s.replace(/\s/g, '!SPACE!')) // preserve spaces in string
        .replace(/([()])/g, ' $1 ')
        .trim()
        .split(/\s+/g)
        .map(x => x.replace(/!SPACE!/g, ' ')); // restore spaces in string
}

const isBoolean = token => token === 'true' || token === 'false';

function atom(token) {
    if (!isNaN(parseFloat(token))) {
        return {type: 'number', value: parseFloat(token)};
    }
    if (token[0] === '"' && token.slice(-1) === '"') {
        return {type: 'string', value: token.slice(1, -1)};
    }
    if (isBoolean(token)) {
        return {type: 'boolean', value: token === 'true'};
    }
    return {type: 'identifier', value: token};
}

function parse(input, list) {
    if (list === undefined) {
        return parse(input, []);
    } else {
        const token = input.shift();
        if (token === undefined) {
            return list;
        } else if (token === "(") {
            list.push(parse(input, []));
            return parse(input, list);
        } else if (token === ")") {
            return list;
        } else {
            return parse(input, list.concat(atom(token)));
        }
    }
}

const $ENV = {
    'first': x => x[0],
    'rest': x => x.slice(1),
    'print': (...x) => console.log(x.join('')),
    '+': (a, b) => a + b,
    '-': (a, b) => a - b,
    '*': (x, y) => x * y,
    '/': (a, b) => a / b,
    '^': (a, b) => a ** b,
    '%': (a, b) => a % b,
    '=': (a, b) => a === b,
    '!=': (a, b) => a !== b,
    '>': (a, b) => a > b,
    '>=': (a, b) => a >= b,
    '<': (a, b) => a < b,
    '<=': (a, b) => a <= b,
    'not': a => !a,
    'or': (a, b) => a || b,
    'and': (a, b) => a && b,
    'is?': (a, b) => a instanceof b,
    'type-of': a => typeof a,
    'args': process.argv.slice(3).map(atom),
    'list': (...a) => a,
    'select': (a, b) => a[b - 1],
    'size': (...a) => a.length
};

class Context {
    scope;
    parentScope;

    constructor(scope, parentScope) {
        this.scope = scope;
        this.parentScope = parentScope;
    }

    find(sym) {
        if (sym in this.scope) {
            return this;
        } else {
            return this.parentScope.find(sym);
        }
    }

    get(sym) {
        return this.scope[sym];
    }

    set(sym, fun) {
        this.scope[sym] = fun;
    }
}

function interpret(input, context = new Context($ENV)) {
    if (input.type === "identifier") {
        return context.find(input.value).get(input.value);
    } else if (input.type === "number" || input.type === "string" || input.type === "boolean") {
        return input.value;
    }
    if (input instanceof Array) {
        const [op, ...args] = input
        if (op.value === 'var') {
            const [/* var */, {value: name}, {value: val}] = input;
            return context[name] = interpret(val, context);
        } else if (op.value === 'function') {
            const [/* function */, {value: name}, {value: params}, {value: body}] = input;
            let f = (...a) => interpret(body, eval_ast_or_bind(params, context, a));
            f.__args__ = params;
            f.__body__ = body;
            f.__env__ = context;
            return context[name] = f;
        } else if (op === 'if') {
            const [/* if */, {value: condition}, {value: true_block}, {value: false_block}] = input;
            return interpret(condition, context) ? interpret(true_block, context) : interpret(false_block, context);
        } else {
            env.find
            let [f, ...a] = eval_ast_or_bind(input, context);
            if (typeof f === 'function') {
                return f(...a);
            }
            return f;
        }
    }
}

export default function $REPL(code) {
    try {
        return interpret(parse(tokenize(clean(code))))
    } catch (e) {
        console.log(`\x1b[31m${e.toString()}\x1b[0m`);
        return null
    }
}

// REPL
if (esMain(import.meta)) {
    const filename = process.argv[2];
    if (filename && filename.endsWith('.rep')) {
        let output = $REPL(readFileSync(filename, 'utf-8'));
        if (output) {
            console.log(output);
            process.exit(0)
        } else {
            process.exit(1)
        }
    }
    console.log("\x1b[93mREPLisp v2.0\x1b[0m by \x1b[95m@pavi2410\x1b[0m");
    console.log("\x1b[94;4mhttps://github.com/pavi2410/REPLisp\x1b[0m");
    start({
        prompt: 'REPLisp> ',
        eval: $REPL
    });
}