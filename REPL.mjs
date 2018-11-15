import Lexer from "./Interpreter/Lexer";
import Parser from "./Interpreter/Parser";
import Interpreter from "./Interpreter/Interpreter";

export default class REPL {
    static run(code) {
        const tokens = Lexer.tokenize(code);
        const ast = Parser.parse(tokens);
        return Interpreter.interpret(ast);
    }
}