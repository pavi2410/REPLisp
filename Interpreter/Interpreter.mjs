function interpret(ast) {

    function call(method) {
        const args = Array.from(arguments).slice(1);
        const [x, y] = args.slice(0, 2);

        switch (method) {
            case '+':
                return args.reduce((a, b) => a + b);
            case '-':
                if (!y) {
                    return -x;
                }
                return x - y;
            case '*':
                return args.reduce((a, b) => a * b);
            case '/':
                return x / y;
            case '%':
                return x % y;
            case '^':
                return x ** y
            // case '!':
            //   return factorial(x)
        }
    }

    function parseExpr(expr) {
        switch (expr.type) {
            case 'NUM':
                return Number(expr.value);
            case 'STR':
                return expr.value;
            case 'CALL':
                return call(expr.name, ...expr.params.map(parseExpr))
        }
    }

    if (ast.type === 'Program') {
        return parseExpr(ast.body[0])
    }
}

export default {interpret};