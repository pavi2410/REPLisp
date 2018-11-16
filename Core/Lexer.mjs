export default class Lexer {
    static tokenize(code) {
        code = code
            .replace(/"[^"]+"/g, m => m.replace(/\s/g, '#SPACE#'))
            .replace(/\(/g, ' ( ')
            .replace(/\)/g, ' ) ')
            .split(' ')
            .filter(x => x)
            .map(x => x.trim().replace(/#SPACE#/g, ' '));

        let tokens = [];

        function addToken(type, value) {
            tokens.push({type, value})
        }

        for (let i = 0; i < code.length; i++) {
            let token = code[i];

            // Parentheses
            if (token === '(' || token === ')') {
                addToken('paren', token);
                continue
            }

            // Boolean
            const boolean = ['true', 'false'];
            if (boolean.includes(token)) {
                addToken('bool', token);
                continue
            }

            // String
            if (/"[^"]+"/.test(token)) {
                addToken('str', token.substring(1, token.length - 1));
                continue
            }

            // Number
            if (/[-\d.]+/.test(token)) {
                addToken('num', token);
                continue
            }

            // Operator
            const operators = ['+', '-', '*', '/', '%', '^', '!', '>', '>=', '<', '<=', '=='];
            if (operators.includes(token)) {
                addToken('op', token);
                continue
            }

            // Keyword
            const keywords = ['fun', 'var', 'set', 'if', 'else'];
            if (keywords.includes(token)) {
                addToken('kw', token);
                continue
            }

            // Identifier
            if (/\w+/.test(token)) {
                addToken('id', token)
            }
        }
        return tokens
    }
}