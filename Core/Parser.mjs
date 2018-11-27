export default class Parser {
    static parse(tokens) {
        let i = 0;

        function walk() {

            let token = tokens.shift();

            // Parentheses
            if (token.type === 'paren' && token.value === '(') {
                token = tokens[++i];

                let node = {
                    type: 'CallExpression',
                    name: token.value,
                    args: []
                };

                token = tokens[++i];
                while (token.type !== 'paren' || token.type === 'paren' && token.value !== ')') {
                    node.args.push(walk());
                    token = tokens[i]
                }

                i++;

                return node
            }

            // Boolean
            if (token.type === 'bool') {
                i++;

                return {
                    type: 'BooleanLiteral',
                    value: token.value
                }
            }

            // String
            if (token.type === 'str') {
                i++;

                return {
                    type: 'StringLiteral',
                    value: token.value.substring(1, token.value.length - 1)
                }
            }

            // Number
            if (token.type === 'num') {
                i++;

                return {
                    type: 'NumberLiteral',
                    value: token.value
                }
            }

            if (token.type === 'kw' && token.value === 'fun') {
                let fun = {
                    type: 'FunctionStatement',
                    name: '',
                    args: [],
                    body: []
                };

                token = tokens[++i];
                if (token.type === 'id') {
                    fun.name = token.value
                }

                token = tokens[++i];
                if (token.type === 'paren' && token.value === '(') {

                    let node = {
                        name: token.value,
                        value: []
                    };

                    token = tokens[++i];
                    while (token.type !== 'paren' || token.type === 'paren' && token.value !== ')') {
                        node.value.push(walk());
                        token = tokens[i]
                    }

                    i++;

                    return node
                }

                return fun
            }

            throw new Error(`I don't know what to do with this token: ${token.value} (${token.type})`);
        }

        let ast = {
            type: 'Program',
            body: []
        };

        while (i < tokens.length) {
            ast.body.push(walk())
        }

        return ast
    }
}