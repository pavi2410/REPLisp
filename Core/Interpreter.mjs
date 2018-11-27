export default class Interpreter {
    static interpret(ast) {

        function call(method) {
            const args = Array.from(arguments).slice(1);
            const [x, y] = args.slice(0, 2);

            switch (method) {
                case '+':
                    return args.reduce((a, b) => a + b);
                case '-':
                    return !y ? -x : x - y;
                case '*':
                    return args.reduce((a, b) => a * b);
                case '/':
                    return x / y;
                case '%':
                    return x % y;
                case '^':
                    return x ** y;
                case '>':
                    return x > y;
                case '>=':
                    return x >= y;
                case '<':
                    return x < y;
                case '<=':
                    return x <= y;
                case '==':
                    return x === y;
                case 'not':
                    return !x;
                case 'or':
                    return x || y;
                case 'and':
                    return x && y;
                case 'if':
                    return x ? y : args[1];
            }
        }

        function parseExpr(expr) {
            switch (expr.type) {
                case 'BooleanLiteral':
                    return expr.value === 'true';
                case 'StringLiteral':
                    return expr.value;
                case 'NumberLiteral':
                    return Number(expr.value);
                case 'CallExpression':
                    return call(expr.name, ...expr.args.map(parseExpr));
                case 'FunctionStatement':
                    // Todo
                case 'VariableStatement':
                    // Todo
            }
        }

        if (ast.type === 'Program') {
            return parseExpr(ast.body[0])
        }
    }
}