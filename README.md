# Graliffer ಠ_ಠ

**Graliffer** is an interpreted exotic programming language using a 2d grid holding both code and data.

Each cells in the grid can contain up to 3 chars (unicode graphems).

Heads walk through the grid reading cells, executing opcodes, and pushing operands to their stack.

![graliffer tui](./graliffer.gif)

## Roadmap to v0.1

Graliffer is new ! Still a lot to cover before the v0.1

- [ ] An interpreter ([`grai`](https://github.com/Ora9/Graliffer/tree/master/crates/grai))
  - [x] Minimal working set
  - [ ] Good error handling
  - [ ] Paralellisation with multiples heads
  - [ ] Absurd programming
  - [ ] Operand type in "Granary" numerical system, to manipulate numbers with an absurdly high base (for counting higher than 999, using the whole unicode set of character)
  - [ ] Differents I/O :
    - [ ] Textual
    - [ ] Graphical
    - [ ] Sound
- [ ] A terminal user interface ([WIP!](https://github.com/Ora9/Graliffer/tree/master/crates/graliffer))
  - [ ] Ergonomic visual editor
    - [x] Keyboard centric editing
    - [ ] Configuration
    - [ ] Undo mechanism,
    - [ ] Copy-pasting,
    - [ ] Address picking et selection
  - [ ] Examples, templates and snippets
  - [ ] Good debuging utilities, stack visualisation, breakpoints, step-by-step ...
