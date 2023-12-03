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

## Day 2

Rust makes a lot of things very elegant, but string parsing is not one of them.
At least, not that I've found to do it. Almost all of my time was spent parsing
stuff.

The approach I'm taking this year is mostly to parse the input into an easily
usable state, then solve the problems with that state. The hardest part of state
parsing today was handling the errors. I wanted to try to have slightly better
error messages, which means I had to put a little but of data in the enum
variants. I spent a decent amount of time hunting down why it wouldn't let me do
that! Turns out I was trying to reference the function input in the errors, and
it didn't like that. As a simplified example:

```rust
fn verify_string_is_cool(value: String) -> Result<Data, Error> {
    if value.contains("cool") {
        Ok(Data("Yup, it's cool."))
    } else {
        Err(Error::Uncool(&value))
    }
}
```

The "happy path" doesn't try to return references to data owned by the function,
but the "unhappy" path does. That threw me off for a bit. Most strongly typed
languages don't ask you to think about the error path, and weakly typed
languages do even less. I think it's better to have to think about it, though.
I'll just have to get used to it.

## Day 3

There's not much to day about this day. Parsing threw a small gotcha at me, but
ended up not being too difficult. Solving the puzzle essentially boiled down to
nested loops.

I started parsing by just splitting on periods, then recording them based on
their symbol or number. That posed a problem when I ran into `617*`: the
aforementioned "gotcha." There's no period, but there are two distinct tokens in
there. Splitting on periods won't split up these two tokens. I had to switch to
a character-by-character approach. I've done that before, I was just hoping to
not have to do it.

I specifically chose to parse the tokens into two separate arrays for a couple
of reasons. First, they're distinct types of tokens. Second, since I was looking
for numbers adjacent to symbols, it made sense to be able to loop over numbers
then check each symbol to see if it was adjacent. This proved to be a good
decision, because for part two I had to loop over symbols then check each number
to see if it was adjacent.

If I _really_ wanted to be fast, I'd do some sort of spatial partitioning thing.
I didn't need to. Even on the full input it runs in 16 ms.
