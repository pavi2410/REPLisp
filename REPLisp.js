const repl = require('repl');
const fs = require('fs');

let Env = Object.assign(this, {
    '+': (...a) => a.reduce((x, y) => x + y),
    '-': (...a) => a[0] - a[1],
    '*': (...a) => a.reduce((x, y) => x * y),
    '/': (...a) => a[0] / a[1],
    '=': (...a) => a[0] === a[1],
    '!=': (...a) => a[0] !== a[1],
    '>': (...a) => a[0] > a[1],
    '>=': (...a) => a[0] >= a[1],
    '<': (...a) => a[0] < a[1],
    '<=': (...a) => a[0] <= a[1],
    'not': (...a) => !a[0],
    'or': (...a) => a[0] || a[1],
    'and': (...a) => a[0] && a[1],
    'list': (...a) => a,
    'is': (...a) => a[0] instanceof a[1],
    'typeof': (...a) => typeof a[0],
    'print': console.log,
    'eval': (...a) => EVAL(a[0], Env),
    'args': process.argv.slice(3),
    'throw': (...a) => { throw(a[0])},
});


function tokenize(lines) {
    lines = lines.replace(/"[^"]*"/g, s => s.replace(/\s/g, '!SPACE!'))
    lines = lines.replace(/([()])/g, ' $1 ')
    lines = lines.split(/\s+/g)
    lines = lines.filter(x => x.trim())
    lines = lines.map(x => x.replace(/!SPACE!/g, ' '))
    return lines
}

function atom(token) {
    if (token in ['true', 'false']) {
        return Boolean(token)
    }
    const num = Number(token);
    if (!isNaN(num)) {
        return num
    }

    if (/"[^"]*"/.test(token)) {
        return token.replace(/"([^"]*)"/, '$1')
    }
    return token
}

function read(tokens) {
    if (tokens.length === 0) {
        return
    }

    // Grab the first token.
    let token = tokens.shift();
    if (token === "(") {
        let list = [];
        while (tokens[0] && tokens[0] !== ")") {
            list.push(read(tokens))
        }
        // Keep going (since we may have nested lists).
        tokens.shift();
        return list
    } else if (token === ")") {
        throw new Error("Unexpected token ')'")
    } else {
        return atom(token)
    }
}

function PARSE(code) {
    const tokens = tokenize(code);
    let ast = [];

    while (tokens.length !== 0) {
        ast.push(read(tokens))
    }
    return ast
}

function eval_ast_or_bind(ast, env, exprs) {
    if (exprs) {
        env = Object.create(env);
        if (Array.isArray(ast)) {
            ast.some((a, i) => a === "&" ? env[ast[i + 1]] = exprs.slice(i) : (env[a] = exprs[i], 0));
        }
        return env;
    }
    // Evaluate the form/ast
    return ast instanceof Array
        ? ast.map((...a) => EVAL(a[0], env))
        : typeof ast == 'string'
            ? ast in env
                ? env[ast]
                : Env.throw(ast + " not found")
            : ast
}

function EVAL(ast, env) {
    while (true) {
        if (!(ast instanceof Array)) return eval_ast_or_bind(ast, env);

        if (ast[0] === "var") {        // update current environment
            return env[ast[1]] = EVAL(ast[2], env)
        } else if (ast[0] === "function") {  // define new function (lambda)
            let f = (...a) => EVAL(ast[3], eval_ast_or_bind(ast[1], env, a));
            f.A = [ast[2], env, ast[1]];
            return env[ast[1]] = f
        } else if (ast[0] === "try") { // try/catch
            try {
                return EVAL(ast[1], env)
            } catch (e) {
                return EVAL(ast[2][2], eval_ast_or_bind([ast[2][1]], env, [e]))
            }
        }

        if (ast[0] === "if") {  // branching conditional
            if (ast[2] !== 'then' && ast[4] !== 'else') {
                throw new Error('Syntax error: if');
            }
            ast = EVAL(ast[1], env) ? ast[3] : ast[4];
        } else {                      // invoke list form
            let [f, ...a] = eval_ast_or_bind(ast, env);
            if (f.A) {
                ast = f.A[0];
                env = eval_ast_or_bind(f.A[2], f.A[1], a)
            } else {
                return typeof f === 'function' ? f(...a) : f;
            }
        }
    }
}

function REPL(code) {
    return EVAL(PARSE(code), Env)
}

// REPL

if (process.argv[2] && process.argv[2].endsWith('.rep')) {
    const file = fs.readFileSync(process.argv[2], 'utf-8');
    try {
        console.log('=>' + REPL(file))
    } catch (e) {
        console.log(e)
    }
    process.exit()
} else {
    console.log(`\x1b[93mREPLisp v2.0\x1b[0m by \x1b[95m@pavi2410\x1b[0m
  \x1b[94;4mhttps://github.com/pavi2410/REPLisp\x1b[0m`);
    repl.start({
        prompt: 'REPLisp> ',
        input: process.stdin,
        output: process.stdout,
        eval: REPL
    });
}
