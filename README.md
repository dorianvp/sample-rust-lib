# UniFFI Starter

This is a simple, mostly minimal, but more-complex-than-just-addition
demonstration of how to use [UniFFI](https://github.com/mozilla/uniffi-rs)
in a structure that works for MOST use cases.

## Motivating use case

It's more complex than addition, but it's intentionally
a bit ridiculous to keep things fun and demonstrate a wide range of uses
in a small project. Note that the vast majority is boilerplate.
Read on for a guide to the "good parts".

We're building a "safe" calculator that can do integer arithmetic.
(The usual demostrations of just addition are not complex enough to be very useful.)
The Rust core will provide addition and division operators.

By "safe" of course I mean that on the Rust side, everything returns a result,
which is translated into exceptions for iOS and Android.
At least you'll get a clean exception from Rust to let you know
about your integer overflows and division by zero!

#### Stuff to look at

- The interesting stuff lives under the `SafeCalculator` and `SafeMultiply` classes.
- Also check out the tests for an example of usage.
