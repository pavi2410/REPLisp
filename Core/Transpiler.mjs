export default class Transpiler {
    static transpile(ast) {
        function call(method) {
            const args = Array.from(arguments).slice(1);
            const [x, y] = args.slice(0, 2);

            switch (method) {
                case '+':
                    return args.reduce((a, b) => a + "+" + b);
                case '-':
                    return !y ? '-' + x : x + '-' + y;
                case '*':
                    return args.reduce((a, b) => a + "*" + b);
                case '/':
                    return x + '/' + y;
                case '%':
                    return x + '%' + y;
                case '^':
                    return x + '**' + y;
                case '!':
                    return 'ReMath.factorial(' + x + ')'
            }
        }

        function parseExpr(expr) {
            switch (expr.type) {
                case 'BOOL':
                    return expr.value;
                case 'STR':
                    return '"' + expr.value + '"';
                case 'NUM':
                    return expr.value;
                case 'CALL':
                    return '(' + call(expr.name, ...expr.args.map(parseExpr)) + ')'
            }
        }

        if (ast.type === 'Program') {
            return parseExpr(ast.body[0])
        }
    }
}