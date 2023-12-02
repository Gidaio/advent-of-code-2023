# Advent of Code 2023

Hello, and welcome to my solutions to Advent of Code 2023! Like last year, I'll
be writing these in Rust to continue to learn the language better, and ideally
I'll write up what I learned and my thought process behind my solution each day.
It worked great last year, and I learned a _ton_ about Rust.

## Day 1
Whew! If I were just trying to solve this real quick in JavaScript or something,
I'd've finished it _way_ faster. But I'm not; I'm trying to do "good practice"
stuff, and also trying to experiment with Rust. It took a little while to get
back into the swing of things, but it's coming back to me.

I decided to try a different structure than I did last year. Since we're
_always_ loading the inputs from a file, that logic could be hoisted up the call
stack. Each puzzle can define its own way of parsing said file by implementing
`TryFrom<File>`. For example, day 1 has a `State` that just holds a
`Vec<String>`. (Honestly, naming it "State" is probably overly grandiose, but
this puzzle is so simple and generic that it was hard to come up with a good
name.) Then it can `impl TryFrom<File> for State`. If all of the puzzles do this
(which is my plan), it allows me to abstract the file loading logic to
`main.rs`, which you can see in the `run_puzzle` function as a trait guard:
`In: TryFrom<File>`.

I'm not the most happy with error handling. My current idea is that every puzzle
will have its own type of error, and as long as they `impl Error` then all the
display stuff should work just fine. In `run_puzzle`, though, I can't just use
the handy `?` operator; I can't just send it up the call stack because it is the
top level. I have to actually handle the errors. `if let` works _great_ with
`Option`s, but not as well with `Result`s. To me, it's because the `None` arm of
the `Option` has no data in it, so its handling pretty naturally fits into the
`else` block. With `Result`s, both arms have associated data, and doing

```rust
if let Ok(foo) = result {
    // ...
} else if let Err(bar) = result {
    // ...
}
```

feels weird. `else` should be for covering the "other" options, but here we have
to explicitly call it out to get the variable binding we need. It's not awful,
just a little awkward. This points to `match` being a "better" option. That got
very nested very fast, so I've instead settled on a "guard" approach.

1. Check if the result is an error.
1. If it is, print it out and exit early.
1. If it's not, well, you can now safely `.unwrap()` it later down the line.

I've also tried using tests this time. The example inputs are usually small
enough to fit inline, so I can skip all the file loading rigamarole. It also
lets me test any smaller functions that I create as part of the solution.

The only comments I have specifically about the solution are about part 2.
Matching single digits is easy enough in Rust with `String::chars()` and
`char::is_digit()`. Matching the digit words was a little trickier. I decided on
the most straightforward approach: try to find every possible meaningful
substring and keep track of the earliest and latest occurrences. A little
parsing utility function later and you've turned either `"0"` or `"zero"` into
`0`. The summation function actually remained the same between part 1 and part 2
which is a sign to me that I conceptually split the problem at the "right"
place. I just duplicated the function, though, because I thought abstracting
which function is called wouldn't be worth it.
