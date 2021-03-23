# Ei-deas

## Intro

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

This document is highly volatile. Right now I'm just making arbitrary
syntax decisions that I'll probably have to clear up later.

The name Ei is German for egg. I don't know what else to tell you.

## What might hello world look like?

```
IO.puts("Hello, world!");
```

## How are bindings defined?

### Immutable

```
let x: String = "Hello"
let y: Bool = true
let z: Int = 500
```

### Mutable

```
mut x: String = "Hello"
mut y: Bool = true
mut z: Int = 500
```

## How are functions on values defined?

### Addition

```
fn add(x: Int, y: Int) -> Int {
    x + y
}
```

### Fizz Buzz

```
fn fizz_buzz(i: Int) -> String {
    mut result = "";

    if i % 3 == 0 {
        result += "Fizz";
    }

    if i % 5 == 0 {
        result += "Buzz";
    }

    if result.is_empty() {
        result = i.to_string();
    }

    result
}
```

## How are types defined & instantiated?

### Enums

```
enum CoffeeMaker {
    aeropress(inverted: Bool),
    espresso,
    kalita_wave,
    v60,
}

let my_favourite = CoffeeMaker.kalite_wave;
```

### Record types

```
record Person {
    name: String,
    age: Int,
}

let me = Person { name: "Matthew", age: 29 };
```

### Interfaces

```
interface Describable {
    fn description() -> String
}
```

## How do we add behaviour to types?

### "Method syntax"

```
impl CoffeeMaker {
    fn is_pourover() -> Bool {
        switch self {
        kalita_wave | v60 => true,
        _ => false,
        }
    }
}
```

```
impl Person {
    fn is_adult() -> Bool {
        self.age > 18
    }
}
```

### Implementing Interfaces

```
impl CoffeeMaker: Describable {
    fn description() -> String {
        "\(self.name) - \(self.age.to_string())"
    }
}
```

## How might we write checked types?

### In Records

```
record Person {
    id: String checking HasFormat("org:user:\[(0-9)+\]"),
    name: String checking NonEmpty,
    age: Int checking StrictlyBetween(0, 130),
}
```

### In enums

```
enum GenderIdentity {
    custom(description: String checking NonEmpty),
    female,
    male,
    nonbinary,
    prefer_not_to_say,
}
```

### In Functions

```
fn divide(numerator: Int, denominator: Int checking NonZero) -> Int {
   numerator / denominator 
}
```

## How might we implement checks?

```
typecheck NonEmpty against String { 
    switch value {
    "" => error,
    _  => okay
    }
}

typecheck StrictlyGreaterThan(lower: Int) against Int {
    switch (value, lower) {
    succ(_), 0 => okay,
    0, succ(_) => error,
    succ(val), succ(lwr) => check val StrictlyGreaterThan(lwr) // TODO: this syntax doesn't really work
    } 
}

typecheck StrictlyLessThan(upper: Int) against Int {
    switch (value, upper) {
    0, succ(_) => okay,
    succ(_), 0 => error,
    succ(val), succ(upr) => check val StrictlyLessThan(upr) // TODO: syntax
    }
}

typecheck StrictlyBetween(lower: Int, upper: Int) against Int {
    // Assume CheckBoth has a signature like Check[T] -> Check[T] -> Check[T]
    check value CheckBoth(StrictlyGreaterThan(lower), StrictlyLessThan(upper))
}
```

Todo:
- How should error messaging be defined?

## Notes

`T checking R` should probably always a strict subtype of `T`, regardless of whether or not `R` actually filters out any values.

