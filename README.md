# Graliffer ಠ_ಠ

**Graliffer** is an interpreted exotic programming language using a 2d grid holding both code and data.

Each cells in the grid can contain up to 3 chars (unicode graphems).

Heads walk through the grid, pushing operands to their stack, and executing opcodes.

## Roadmap

- [x] A interpreter library ([WIP at `grai`](https://github.com/Ora9/Graliffer/tree/master/crates/grai))
- [ ] A terminal user interface ([WIP!](https://github.com/Ora9/Graliffer/tree/master/crates/graliffer))
  - [ ] An visual editor with ergonomics in mind : undo mechanism, copy-pasting, multi-cell selection, address picking et selection..
  - [ ] Examples, templates and snippets
  - [ ] Good debuging utilities, stack visualisation, breakpoints, step-by-step ...
  - [ ] Differents I/O, textual, graphical, sound...
- [ ] Paralellisation with multiples heads
- [ ] New opcodes for absurd programming
- [ ] A new operand to manipulate numbers with an absurdly high base for counting higher than 999, using the whole unicode set of character
