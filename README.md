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

## Day 4

I've really been learning a ton about `Result`s. Also iterators. Iterators are
amazing, and I've been looking for places to insert them into my JavaScript/
TypeScript, too! Rust just has so many utilities for it that they're so easy to
use. Take, for example:

```rust
let winning_numbers = winning_numbers_str
    .split(" ")
    .filter_map(|number| {
        if number.len() == 0 {
            None
        } else {
            Some(number.parse::<usize>())
        }
    })
    .collect::<Result<HashSet<_>, _>>()?;
```

The `filter_map` alone does a ton:

1. It splits the string on spaces. (`.split(" ")`)
1. It filters out empty strings, since some numbers have two spaces before them.
   (`if number.len() == 0 { None }`)
1. It parses what's left into a `usize`.

And that's just the `filter_map`! After that's all done, we're left with an
iterator of `Result<usize>`. The final bit of "magic" is `.collect`.

First, `collect` can create any container that implements `FromIterator`. Up
until now, I'd only ever done `Vec`s, but I wanted a set this time to make use
of the fancy `intersection` function. I tried this out and it worked, but I
didn't know why until I did some research. The cool part is, this means you can
make your own types that you can `collect` into!

Naively, `collect` would turn an iterator of `Result<T>`s into a collection of
`Result<T>`s, i.e. `Vec<Result<T>>`. That's not often useful, so `collect` can
"invert" that into a `Result` of the collection, i.e. `Result<Vec<T>>`. It's
super handy. That means the final line is doing a lot of work, but it's written
so simply!

I'm comfortable enough with Rust now to, with a little effort, write things like

```rust
Ok(Puzzle(
    value
        .lines()
        .map(|line| line.try_into())
        .collect::<Result<Vec<_>, _>>()?,
))
```

Previously, I would've busted my brain trying to figure out how to do that. I
just did it in one (semantic) line!

I love Rust. It's so much fun. I just keep coming back to one word when I try to
describe it: "expressive".

## Day 5

The parsing for this one was slightly interesting. I probably could've just
split on `\n\n` and then handled each map, but I decided to do a more "recursive
descent" type parser. A trick I learned last year is to use `while let` loops
instead of `for` loops. `for` loops borrow the _iterator_ for the entire
duration of the loop, so you can't call `.next()` inside of the loop.
`while let` loops don't borrow the iterator, only the result of the `.next()`
call.

The "part 2 twist" was a doozy for this one. Changing from mapping numbers to
entire ranges required a lot of rethinking. Eventually it coalesced into
something that I'm really happy with. The entire process mostly hinges on two
functions.

`MapEntry::map_range` takes a range and returns a 3-tuple
`(Range, Option<Range>, Option<Range>)` corresponding to the mapped range, any
leftovers on the left, and any leftovers on the right. It has to handle all nine
cases of two ranges overlapping.

`map_range_recursive` takes a bunch of `MapEntry`s and a `Range` to map. As you
might expect, it's recursive. It first finds a `MapEntry` that overlaps with the
range. (This is actually a potential bug and I'm lucky it didn't bite me in the
butt. It's _possible_ that a single `Range` overlaps with multiple `MapEntry`s.)
Then it calls itself recursively on any left and right straggling ranges. Once
that's done, it compiles them all into a `Vec` and returns that.

The core of the algorithm is the same as part 1, just mapping ranges instead of
mapping single numbers. Then it just pulls out the lowest value of each range
and returns the minimum.

I was worried I'd have to do a pass between each mapping to merge ranges to keep
the `Vec`s reasonably sized. Each mapping could theoretically fracture each
range into _three_ (or maybe more!) ranges. Fortunately, I think the puzzle
design is nice and doesn't proliferate too much.

## Day 6

There was exactly one interesting thing today: trying to concatenate numbers for
part 2.

Taking a `Vec<isize>` and converting it into a single `isize` was an interesting
problem. It's the first time I'd approached it in Rust. The "standard" way to do
this is to count the number of digits in each value and accumulate them up. When
every number is one digit, it's easy.

```rust
vec![1, 2, 3, 4, 5].iter().reduce(|acc, val| acc * 10 + val);
```

The biggest challenge isn't the accumulation, it's counting how many digits are
in the number. I eventually settled on repeatedly dividing and multiplying by
10.

```rust
fn get_digit_multiplier(num: isize) -> isize {
    let mut multiplier = 10;
    let mut num = num;
    while num >= 10 {
        num /= 10;
        multiplier *= 10;
    }

    multiplier
}
```

That worked great. By directly calculating the multiplier instead of the number
of digits, I avoided having to do a power (`10 ^ n`) afterwards.

I think because I'm doing things in Rust, the difficulty curve is wonky. Some
things that involve lots of string parsing are harder, and other things are way
easier. Today was done in like, 15 minutes.
