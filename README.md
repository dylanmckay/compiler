
# An optimising compiler middle/back end

[![Build Status](https://travis-ci.org/dylanmckay/compiler.svg)](https://travis-ci.org/dylanmckay/compiler)

This program is written in Rust and runs on the nightly channel.

It is a compiler middle end, with its own IR and optimization passes. The goal
is to get a small number of optimizations working, implement an IR parser, and
then to write a backend for some architecture (probably x86 or AVR).

It is similar to LLVM, except very much a work in progress. The end goal is
to have an easy-to-use compiler library leveraging Rust's features, with
fairly fast generated programs.

