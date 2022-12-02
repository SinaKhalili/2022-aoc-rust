# aoc to learn rust!

or will I just be a ` usafe {  } ` rust noob forever?

BORROWCHECKING: ON.
COPILOT: OFF.

## input
Opening files? Whose got time for that? I'll just be feeding on stdin

```
cat datx.txt | cargo run
```

## notes
day 1: 
 - read the first bit of [the rust book](https://doc.rust-lang.org/book/) but no borrow checking or anything
 - `loop` is an interesting keyword
 - const by default is nice I guess
 - `&` as a reference -> "don't copy me"
 - `vec!` is a macro that creates a vector
 - there's `Result` and there's `Option` for return types
    - `Result` is for when you want to return an error (Ok or Err)
    - `Option` is for when you want to return `None` (None or Some), reminds me of Maybe in Haskell or the Optional type in Python
    - you can `.unwrap()` a `Result` or `Option` to get the value out, but it will panic if it's `Err` or `None`
 - You can do `parse::<TYPE>` to parse into a type. `parse::<i32>()` will parse into an integer. I think this is like a template function in C++.

so far, so good.