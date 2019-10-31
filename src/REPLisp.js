const {start} = require('repl');
const {readFileSync} = require('fs');

const tokenize = lines => lines
    .replace(/#.*$/g, '')
    .replace(/"[^"]*?"/g, s => s.replace(/\s/g, '!SPACE!'))
    .replace(/([()[\]])/g, ' $1 ')
    .trim()
    .split(/\s+/g)
    .map(x => x.replace(/!SPACE!/g, ' '));

const isBoolean = token => token === 'true' || token === 'false';

const isNumber = token => /^-?\d+(\.\d+|e)?$/.test(token);

const isString = token => /"[^"]*?"/.test(token);

function atom(token) {
    if (isBoolean(token)) return token === 'true';
    if (isNumber(token)) return Number(token);
    return token
}

function read(tokens) {
    if (tokens.length === 0) return;

    let token = tokens.shift(); // Grab the first token.
    switch (token) {
        case '(':
            let list = [];
            while (tokens[0] && tokens[0] !== ')') {
                list.push(read(tokens))
            }
            tokens.shift(); // Keep going (since we may have nested lists).
            return list;
        case '[':
            let values = [];
            while (tokens[0] && tokens[0] !== ']') {
                values.push(read(tokens))
            }
            tokens.shift(); // Keep going (since we may have nested lists).
            return values;
        case ']':
        case ')':
            throw new SyntaxError("Unexpected token: )");
        default:
            return atom(token)
    }
}

function $parse(tokens) {
    let ast = [];

    while (tokens.length !== 0) {
        ast.push(read(tokens))
    }
    return ast
}

const $ENV = {
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
    'typeof': a => typeof a,
    'print': (...a) => console.log(a.join('')),
    'args': process.argv.slice(3).map(atom),
    'listof': (...a) => a,
    'select': (a, b) => a[b - 1],
    'size': (...a) => a[0][0]
};

function eval_ast_or_bind(ast, env, args) {
    if (args) {
        env = Object.create(env);
        if (ast instanceof Array) {
            ast.some((a, i) => a === '&' ? env[ast[i + 1]] = args.slice(i) : (env[a] = args[i], 0));
        }
        return env;
    }
    // Evaluate the form/ast
    if (ast instanceof Array) {
        return ast.map(a => $eval(a, env))
    }
    if (typeof ast == 'string') {
        if (ast in env) {
            return env[ast]
        }
        if (isString(ast)) {
            return ast.substring(1, ast.length - 1) // Remove quotes
        }
        throw new ReferenceError(`${ast} not found`)
    }
    return ast
}

function process_var(statement, env) {
    const [/* var */, id, val] = statement;
    return env[id] = $eval(val, env);
}

function process_function(statement, env) {
    const [/* function */, id, params, body] = statement;
    let f = (...a) => $eval(body, eval_ast_or_bind(params, env, a));
    f.__args__ = params;
    f.__body__ = body;
    f.__env__ = env;
    return env[id] = f;
}

function process_if(statement, env) {
    const [/* if */, condition, /* then */, true_block, /* else */, false_block] = statement;
    return $eval(condition, env) ? $eval(true_block, env) : $eval(false_block, env);
}

function invoke_function(ast, env) {
    let [f, ...a] = eval_ast_or_bind(ast, env);
    if (typeof f === 'function') {
        return f(...a);
    }
    return f;
}

function $eval(ast, env) {
    if (ast instanceof Array) {
        switch (ast[0]) {
            case 'var':
                return process_var(ast, env);
            case 'function':
                return process_function(ast, env);
            case 'if':
                return process_if(ast, env);
            default:
                return invoke_function(ast, env)
        }
    } else {
        return eval_ast_or_bind(ast, env);
    }
}

function $REPL(code) {
    try {
        return $eval($parse(tokenize(code)), $ENV)
    } catch (e) {
        console.log(`\x1b[31m${e.toString()}\x1b[0m`)
    }
}

// REPL
if (require.main === module) {
    const filename = process.argv[2];
    if (filename && filename.endsWith('.rep')) {
        console.log($REPL(readFileSync(filename, 'utf-8')));
        return;
    }
    console.log(`\x1b[93mREPLisp v2.0\x1b[0m by \x1b[95m@pavi2410\x1b[0m`);
    console.log(`\x1b[94;4mhttps://github.com/pavi2410/REPLisp\x1b[0m`);
    start({
        prompt: 'REPLisp> ',
        input: process.stdin,
        output: process.stdout,
        eval: $REPL
    });
}

module.exports = {$REPL};