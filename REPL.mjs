import Lexer from "./Core/Lexer";
import Parser from "./Core/Parser";
import Interpreter from "./Core/Interpreter";
import Transpiler from "./Core/Transpiler";

export default class REPL {
    static run(code, {debug = false, transpile = false} = {}) {
        const tokens = Lexer.tokenize(code);
        const ast = Parser.parse(tokens);
        if (debug) {
            console.log('\x1b[35m', 'Tokens:', '\x1b[33m', JSON.stringify(tokens, null, '\t'), '\x1b[0m');
            console.log('\x1b[35m', 'AST:', '\x1b[33m', JSON.stringify(ast, null, '\t'), '\x1b[0m');
        }
        if (transpile) {
            const genCode = Transpiler.transpile(ast);
            console.log('\x1b[35m', 'Generated JS Code:', '\x1b[33m', genCode, '\x1b[0m');
        }
        return Interpreter.interpret(ast);
    }
}