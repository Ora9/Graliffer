# Graliffer Standard

This document is intended to be a reference and standard for the Graliffer programming language.

Graliffer is an interpreted exotic programming language using a 2 dimensional grid to hold code and data

# Memory

Graliffer uses two separate memory models :
- a *grid* of cells that can hold up to 3 characters
- a *stack*, holding operands that have been parsed and waiting to be popped by operations

## Grid

Graliffer uses a 2 dimensional grid of cells that can hold up to 3 characters

That way every words, opcodes or operation, in the language has a maximum of 3 char

### Position

The two axis of the grid currently uses `([A-Z][a-z][0-9]\+\/)`

A specific cell in the grid can be referenced by 

See more in chapters [Address](#address) and [Granary](#granary)

## Stack

The stack only hold operands

The stack can be accessed in two way :
- **pop**, when an operand is removed from the end of the stack, and returned to be used
- **push**, when an operand is insert at the end of the stack

# Granary

**Granary** is the numeral system used by Graliffer. It is a positional system of radix 64

Granary's goal is to express large numbers while still minimizing the amount of character used, by utilitizing a large digit set

> [!note] For now granary uses [base64](https://en.wikipedia.org/wiki/Base64) but would happily extend to the whole unicode character set for very large number representation

These range follow each others to represent 0 through 63:
- `[A-Z]`
- `[a-z]`
- `[0-9]`
- `+`
- `/`

# Language

Each cell contains a graliffer *word*

## Operands

*Operands* can be interpreted to gain meaning

### Literal

A *literal*

Operations can choose to interpret literals with specific conventions

#### As booleans

Cells can be understood as boolean with :
- `0` and empty cell are false
- everything else is true

#### As integers

Cells can be understood as integers with theses rules :
- Base 10 (decimal) numeral system
- Numbers in range 0 through 999 can be represented (max 3 char)
- Negative numbers are non-representable
- Cell must contain only digits (0 through 9)
- Leading zeros are ignored

### Address

An *address* represent a position in the *grid*

Operations can choose to use an address to either :
- Point to a literal in the grid, to add a level of indirection, or to escape operand parsing
- Point to a specific position in the grid (for `jmp`, `set`, ...)

Address are parsed with :
- Prefix `@` (at character)
- 2 [Granary](#granary) number (one for each axis, horizontal then vertical)

Examples :
- `@AA` pointing to cell at origin (top left)
- `@EG` pointing to cell at horizontal 5 and vertical 7
- `@5a` > `@AB` results in literal `@AB`
- `@+8`

### Pointer

A *pointer* add a level of indirection, pointing to another operand that will be parsed and interpreted again

Multiple pointers can be chained that way

Pointer are parsed with :
- Prefix `&` (ampersand character)
- 2 [Granary](#granary) number (one for each axis, horizontal then vertical)

Example :
- `&HA` pointing to cell `abc` result in literal `abc`
- `&HA` > `@HB` > `abc` results in `abc`
- `&HA` > `&HB` > `&HC` > `@HD` > `abc` results to `abc`

## Opcodes

*Opcodes* are operations that manipulate data, change code flow, take input or produce output

There exists a large range of opcodes

### Head movements

Change executing head *direction* :
- `gou` : go *up*
- `gor` : go *right*
- `god` : go *down*
- `gol` : go *left*

Pop as address from the stack and move executing head to that address :
- `jmp` ("jump")

Pop one as bool and one as address, if the boolean is truey, jump to the address
- `jif` ("jump if")

### Arithmetics and Comparaison

Pop two as integer, perform *operation* on them and push the result :
- `add` : *add*
- `sub` : *subtract*
- `mul` : *multiply*
- `div` : *divide*

Pop two as literal, *compare* them and push the result :
- `equ` : *equal*
- `neq` : *not equal*

Pop two as integer, *compare* them and push the result :
- `grt` : *greater than*
- `lst` : *less than*
- `grq` : *greater or equal than*
- `lsq` : *less or equal than*

Pop one as bool, negate it, and push the result :
- `not`

### Program management

Halt the program, stopping all heads :
- `hlt` ("halt")
