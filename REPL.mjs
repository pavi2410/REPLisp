import Lexer from "./Core/Lexer";
import Parser from "./Core/Parser";
import Interpreter from "./Core/Interpreter";
import Transpiler from "./Core/Transpiler";

export default class REPL {
    static run(code, {debug = false, transpile = false} = {}) {
        const tokens = Lexer.tokenize(code);
        const ast = Parser.parse(tokens);
        if (debug) {
            console.log('\x1b[35mTokens:\x1b[33m', JSON.stringify(tokens, null, 2), '\x1b[0m');
            console.log('\x1b[35mAST:\x1b[33m', JSON.stringify(ast, null, 2), '\x1b[0m');
        }
        if (transpile) {
            const genCode = Transpiler.transpile(ast);
            console.log('\x1b[35mGenerated JS Code:\x1b[33m', genCode, '\x1b[0m');
        }
        return Interpreter.interpret(ast);
    }
}