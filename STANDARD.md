# Graliffer Standard

This document is intended to define a standard of the Graliffer programming language.

Graliffer is an exotic programming language using a 2 dimensional grid to hold code and data

# Memory

Graliffer uses two separate memory types :
- a *grid*
- a *stack*, holding operands after having being parsed

# Words

Each cell contains a graliffer *word*

## Operands

*Operands*

### Literal

A *literal*

#### Interpretation

A *literal* can be interpreted to gain meaning, based on opcodes :

##### Boolean



##### Numeric

### Address

An *address* represent a position in the *grid*

This *operand*, depending on the *opcode* it is used in, can have two meaning and evaluation :
- To add a level of indirection to another operand
- To represent a certain position in the grid



### Pointer



## Opcodes

*Opcodes* are operations used to manipulate data, change code flow, take input or produce output

### Arithmetic
