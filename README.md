# Introduction

Ei is a small experimental programming language which eventually aims to either
implement a simple form of dependent typing within a modern "C-style" language
or die trying.

The idea is to have two separate langauges exist side-by-side: the main
imperative language in which application code is written, and a subset of that
language comprising pure & total functions which operate on types and value
literals. This second language is used at compile time to extend the
type-checker to, e.g., confirm that a value is within a given range.

My hope is that proofs of these various checks can be generated while
compiling the imperative language. E.g. we can convert an expression like
`if x < 5 { do_something(x) } else { IO.puts("x was too big") }` into a
proof that `x` must be `Int checking StrictlyLessThan(5)` in the first
branch.

