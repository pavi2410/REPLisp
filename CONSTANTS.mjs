export const VERSION = '1.1';

export const HEADER = `\x1b[33;1mREPLisp v${VERSION}\x1b[0m by \x1b[35;1m@pavi2410\x1b[0m
- Type \x1b[32m'help'\x1b[0m for usage and syntax
- Hit \x1b[37m Ctrl + C \x1b[0m to exit
`;

export const USAGE = `
Usage:
    > [COMMAND]

 command         | description
-----------------|---------------------
 <EXPRESSION>    | Evals the expression
 help            | Show this help text

Syntax:
    (<OPERATOR> [<EXPRESSION> ...])

    where <OPERATOR> is one of:
        + (add)
        - (subtract)
        * (multiply)
        / (divide)
        % (remainder)
        ^ (power)
        ! (factorial)
    and <EXPRESSION> can be either a <NUM>, <STRING>, or <EXPRESSION> itself
`.replace(/([<[]\w+[>\]])/gm, `\x1b[36m$1\x1b[0m`)
    .replace(/(.) \((\w+)\)/gm, `\x1b[32m$1\x1b[0m (\x1b[35m$2\x1b[0m)`);