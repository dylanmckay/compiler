# An optimising compiler middle/back end

[![Build Status](https://travis-ci.org/dylanmckay/compiler.svg)](https://travis-ci.org/dylanmckay/compiler)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)

This program is written in Rust and runs on the nightly channel.

It is a compiler middle end, with its own IR and optimization passes. The goal
is to get a small number of optimizations working, implement an IR parser, and
then to write a backend for some architecture (probably x86 or AVR).

It is similar to LLVM, except very much a work in progress. The end goal is
to have an easy-to-use compiler library leveraging Rust's features, with
fairly fast generated programs.

## Status

So far the only target supported is [AVR](https://en.wikipedia.org/wiki/Atmel_AVR).

The compiler can currently parse IR and generate assembly code for very, very basic programs (think very trivial addition and load).

## Running the compiler

There is a bundled tool named `asm` which does most of the work.

Example:

``` bash
cargo run --bin asm -- input.ir
```

## Intermediate representation

This is the IR that is fed into the compiler.
The IR syntax is inspired from Rust and LLVM.

```
fn @main(%foo: i8) {
  %a = add i8 5, i8 10
  %b = add %foo, %a
  %c = sub %foo, i8 5
  ret
}
```

There are several passes implemented on IR. This includes:

* Constant folding
* Dead code elimination

## Middle intermediate representation

Once the IR has been optimised, it is converted into 'MIR' form. This IR is
tree-based. There are two types of nodes: `mir::Branch` and `mir::Leaf`.

`mir::Branch` corresponds to an _operation_. A branch is described by an opcode
and a set of operands (which are themselves nodes).

`mir::Value` corresponds to a simple value such as a reference to a register, a constant
integer, a reference to an argument, etc.

Example:

```
(set %a, i8 5) # load an 8-bit immediate into %a
(set %b, (add %a, i8 1))
```

## Instruction selection

When the MIR is first built, it is _expanded_ into tree form. This collapses
all of the nodes. In the above sample, the expander would recognize that `%a`
is only used once, so the first node can effectively be inlined into the second node.

This code:

```
(set %a, i8 5) # load an 8-bit immediate into %a
(set %b, (add %a, i8 1))
```

Would be expanded to:


```
(set %b, (add i8 5, i8 1)
```

### Pattern matching

Each target defines a set of patterns which match with MIR nodes.

Given a node, the selector works by finding all patterns which either _perfectly_ or
_partially_ match the MIR tree.

If a pattern _perfectly_ matches the tree, the the instruction will be chosen to replace
the node.

If the pattern partially matches, it returns a list of _adjustments_ which specifies _how_ to
change the MIR in order to form a perfect match.

For all partially matching patterns, all of the adjustments are applied to each, which forms
a list of _permutations_. After all of the adjustments are applied, all of the permutations will
either perfectly match or not match at all. The most optimal permutation is then chosen, and then
the node is replaced with the instruction.

## Register allocation

Currently, the only register allocator is _really dumb_. It doesn't try to spill any
registers, and will panic once there are no registers free. Registers are not reused.

## Test suite

The test suite exists in the `tests/` directory. The file format is very similar to LLVM's
`lit` tool.

The entire test suite can be ran by executing

``` bash
cargo run --bin test -- tests
```

