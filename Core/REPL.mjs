import Parser from "./Parser.mjs";
import Interpreter from "./Interpreter.mjs";

export default class REPL {
    static run(code, debug = false) {
        const ast = Parser.parse(code);
        if (debug) {
            console.log('\x1b[35mAST:\x1b[33m', JSON.stringify(ast, null, 2), '\x1b[0m');
        }
        return Interpreter.interpret(ast);
    }
}