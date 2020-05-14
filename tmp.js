const isNumber = token => /^-?\d+(\.\d+|e)?$/.test(token);
const isString = token => /"[^"]*?"/.test(token);

function atom(token) {
    if (isBoolean(token)) return token === 'true';
    if (isNumber(token)) return Number(token);
    // if (isString(token)) return token.slice(1, -1)
    return token
}

function read(tokens) {
    if (tokens.length === 0) return;

    let token = tokens.shift(); // Grab the first token.
    if (token === '(') {
        let list = [];
        while (tokens[0] && tokens[0] !== ')') {
            list.push(read(tokens))
        }
        tokens.shift(); // Keep going (since we may have nested lists).
        return list;
    } else if (token === ')') {
        throw SyntaxError("Unexpected token: )");
    } else {
        return atom(token)
    }
}

function eval_ast_or_bind(ast, env, args) {
    if (args) {
        env = Object.create(env);
        if (ast instanceof Array) ast.some((a, i) => a === '&' ? env[ast[i + 1]] = args.slice(i) : (env[a] = args[i], 0));
        return env;
    }
    // Evaluate the form/ast
    if (ast instanceof Array) return ast.map(a => $eval(a, env));
    if (typeof ast == 'string') {
        if (ast in env) return env[ast];
        if (isString(ast)) return ast.substring(1, ast.length - 1);
        throw ReferenceError(`${ast} not found`)
    }
    return ast
}

function $parse(tokens) {
    if (tokens[0] !== '(') {
        throw SyntaxError("Invalid program. Must start with (")
    }

    return parse(tokens)
}