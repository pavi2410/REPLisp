export default class Parser {
    static tokenize(code) {
        return code
            .replace(/"[^"]+"/g, m => m.replace(/\s/g, '#SPACE#'))
            .replace(/\(/g, ' ( ')
            .replace(/\)/g, ' ) ')
            .split(' ')
            .filter(x => x)
            .map(x => x.trim().replace(/#SPACE#/g, ' '))
    }

    static parse(code) {
        const tokens = this.tokenize(code);

        let i = 0;

        function walk() {

            let token = tokens.shift();

            // Parentheses
            if (token === '(') {
                token = tokens[++i];

                let node = {
                    type: 'CallExpression',
                    name: token.value,
                    args: []
                };

                token = tokens[++i];
                while (token.value !== ')') {
                    node.args.push(walk());
                    token = tokens[i]
                }

                i++;

                return node
            }

            // Boolean
            if (token in ['true', 'false']) {
                i++;

                return {
                    type: 'BooleanLiteral',
                    value: token.value
                }
            }

            // String
            if (/"[^"]"/.test(token)) {
                i++;

                return {
                    type: 'StringLiteral',
                    value: token.value.substring(1, token.length - 1)
                }
            }

            // Number
            if (/-?[\d.]+/.test(token)) {
                i++;

                return {
                    type: 'NumberLiteral',
                    value: token
                }
            }

            if (token === 'fun') {
                let fun = {
                    type: 'FunctionStatement',
                    name: '',
                    args: [],
                    body: []
                };

                token = tokens[++i];
                // identifier
                if (/[A-z]\w+/.test(token)) {
                    fun.name = token
                }

                token = tokens[++i];
                if (token === '(') {

                    let node = {
                        name: token,
                        value: []
                    };

                    token = tokens[++i];
                    while (token !== ')') {
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