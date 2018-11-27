import Lexer from "./Lexer.mjs";
import Parser from "./Parser.mjs";
import Interpreter from "./Interpreter.mjs";

export default class REPL {
    static run(code, debug = false) {
        const tokens = Lexer.tokenize(code);
        const ast = Parser.parse(tokens);
        if (debug) {
            console.log('\x1b[35mTokens:\x1b[33m', JSON.stringify(tokens, null, 2), '\x1b[0m');
            console.log('\x1b[35mAST:\x1b[33m', JSON.stringify(ast, null, 2), '\x1b[0m');
        }
        return Interpreter.interpret(ast);
    }
}