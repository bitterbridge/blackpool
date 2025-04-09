# Blackpool

Blackpool is an experimental programming language, VM, and domain-specific library.

This work is based on **Lox**, from Robert Nystrom's amazing [Crafting Interpreters](https://craftinginterpreters.com), and specifically his bytecode-based CLox, but I'm customizing it heavily based on my personal preferences and intended use case.

It should be noted, though, that I'm less interested in matching the reference implementation of Lox than I am in incorporating a full-featured and flexible scripting language.  I have slightly different syntax in mind than Bob has for Lox, and I'm going to add some domain-specific extensions, etc.   So **TL;DR**: if something's weird or ugly or broken, it's me and not Bob.  All hail Bob.  All... hail... Bob.

```bash
$ cargo test
...
     Running unittests src/lib.rs (target/debug/deps/blackpool-77bee6fba0f16966)
...
test result: ok. 18 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.04s

     Running unittests src/main.rs (target/debug/deps/blackpool-81ecad9f28bc488f)
...
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s

     Running tests/scripting.rs (target/debug/deps/scripting-b505d40b21e0e715)
...
test result: ok. 237 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.39s
...
   Doc-tests blackpool
test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.95s
```

```bash
$ cargo run
...
> print 3 + 4 * 2;
11
OK
> fun f() { var a = "a"; var b = "b"; fun g() { print b; print a; } g(); } f();
b
a
OK
```
