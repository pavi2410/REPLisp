export const VERSION = '0.8';

export const HEADER = `\x1b[33;1mREPLisp v${VERSION}\x1b[0m by \x1b[35;1m@pavi2410\x1b[0m
(Type 'help' for usage and syntax)
`;

export const USAGE = `
Usage:
  > [command]

command          | description
-----------------|---------------------
run <expression> | Evals the expression
test             | Run tests
help             | Show this help text
bye              | Exit

Syntax:
  (<operator> [<argument> ...])

  where <operator> is one of:
    + (add)
    - (subtract)
    * (multiply)
    / (divide)
    % (remainder)
    ^ (power)
  and <argument> can be either a number or an expression itself
`;