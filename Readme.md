# JSON parser

This is a very small-ish example of a relatively simple JSON parser that parses a given json input.
Currently tested on a M3 Macbook Air, it parses a 25MB large json file in ~0.3 seconds on repeat 
benchmarks (maybe because of hot fs, haven't benchmarked that aspect yet).

Not for production use.
Implemented:
- Custom lexer
- Custom recursive-descent parser with some lookahead (to prevent left-recursion, the parser looks
ahead on a token to determine what the next output should be)

Enjoy!
