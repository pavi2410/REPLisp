export default class Lexer {
    static tokenize(code) {
        let tokens = [];

        function addToken(type, value) {
            tokens.push({type, value})
        }

        code = code
            .replace(/"[^"]+"/g, m => m.replace(/\s/g, '#SPACE#'))
            .replace(/\(/g, ' ( ')
            .replace(/\)/g, ' ) ')
            .split(' ')
            .filter(x => x)
            .map(x => x.trim().replace(/#SPACE#/g, ' '));

        for (const token of code) {

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
                addToken('str', token);
                continue
            }

            // Number
            if (/-?[\d.]+/.test(token)) {
                addToken('num', token);
                continue
            }

            // Operator
            const operators = ['+', '-', '*', '/', '%', '^', '!', '>', '>=', '<', '<=', '==', 'not', 'or', 'and'];
            if (operators.includes(token)) {
                addToken('op', token);
                continue
            }

            // Keyword
            const keywords = ['fun', 'set', 'if', 'else', 'print'];
            if (keywords.includes(token)) {
                addToken('kw', token);
                continue
            }

            // Identifier
            if (/\w+/.test(token)) {
                addToken('id', token);
                continue
            }

            throw new Error("I don't know what this token is: " + token);
        }
        return tokens
    }
}