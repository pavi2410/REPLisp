function tokenize(code) {
    let tokens = [];

    function addToken(type, value) {
        tokens.push({type, value})
    }

    for (let i = 0; i < code.length; i++) {
        let char = code[i];

        if (/\s/.test(char)) continue;

        // Parens
        if (char === '(' || char === ')') {
            addToken('paren', char);
        }

        // String
        else if (char === '"') {
            let value = '';
            char = code[++i];

            while (char !== '"') {
                value += char;
                char = code[++i];
            }

            char = code[++i];
            i--;

            addToken('str', value);
        }

        // Number
        else if (/\d/.test(char)) {
            let value = char;
            char = code[++i];

            while (/\d/.test(char) || char === '.') {
                value += char;
                char = code[++i];
            }
            i--;

            addToken('num', value);
        }

        // Operators
        else if (char === '+' || char === '-' ||
            char === '*' || char === '/' ||
            char === '%' || char === '^' ||
            char === '!' || char === '-') {
            addToken('op', char);
        }
    }

    return tokens
}

export default {tokenize};