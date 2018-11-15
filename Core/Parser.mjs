export default class Parser {
    static parse(tokens) {
        let i = 0;

        function walk() {

            let token = tokens[i];

            if (token.type === 'num') {
                i++;

                return {
                    type: 'NUM',
                    value: token.value
                }
            }

            if (token.type === 'str') {
                i++;

                return {
                    type: 'STR',
                    value: token.value
                }
            }

            if (token.type === 'paren' && token.value === '(') {
                token = tokens[++i];

                let node = {
                    type: 'CALL',
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