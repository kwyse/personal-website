+++
title = "Book Recommendation: Programming Rust"
author = ["Krishan Wyse"]
publishDate = 2018-02-18T00:00:00+00:00
categories = ["reading"]
draft = false
+++

Most of us can agree that Rust's learning curve is steep. I've been using it for
hobby projects for the last few years but I'm still hesitant to use it when I'm
constrained _by a deadline_, because of the upfront cost in development
time. And that's frustrating, because I really believe that the benefits it
offers outweigh that.

[_Programming Rust_](http://shop.oreilly.com/product/0636920040385.do) is included in the latest Humble _Functional Programming_
[book bundle](https://www.humblebundle.com/books/functional-programming-books). Take a look on [Amazon](https://www.amazon.com/product-reviews/1491927283/ref=cm%5Fcr%5Fdp%5Fd%5Fcmps%5Fbtm?ie=UTF8&reviewerType=all%5Freviews) and you'll see it's getting some pretty rave
reviews. It's the first book that's come my way focused on Rust end-to-end.

Personally, I find reading a book like this from cover to cover the best way to
really _learn_ a language. You need to put it into practice with projects, of
course, but projects don't teach you idioms, best practices, optimisation areas,
and language-specific quirks that you should be aware of. Neither does using the
language day in and day out at work, necessarily. This knowledge comes from
experts.

Just like [_K&R_](https://en.wikipedia.org/wiki/The%5FC%5FProgramming%5FLanguage), _C++: The Complete Reference_, _Java: The Complete Reference_,
and _Programming Ruby_ before it gave me insight into their respective
languages, I'm hopeful _Programming Rust_ can continue the trend.


## Initial impression {#initial-impression}

So far it's exceeded expectations! The book is broken down into four sections:

1.  Fundamentals: an overview of the language and details of the borrow checker
2.  Language constructs: expressions, error handling, and the module system
3.  Traits and generics
4.  Advanced and use case-specific Rust: IO, concurrency, and `unsafe` code

I've completed the first section, comprising five chapters. The last two
chapters are about ownership and references respective. They offer the best
explanations of both topics I've come across.

The ownership chapter explains Rust's move semantics. Comparisons to C++ and
Python give reasons for why Rust's approach was chosen and the benefits of each
approach, but also how to recreate the other two in Rust should you need to.

The references chapter explains lifetimes and the rules the borrow checker
enforces, and repeats them in different scenarios so that they sink in. But by
far the most valuable part of this chapter are the diagrams explaining how
references are laid out in memory. They won't come as a shock, as the memory
model is intuitive and what you would expect, but seeing them illustrates _why_
the borrow checker complains when it does. These diagrams have been the
highlight of the book thus far.

I'm satisfied that my understanding of Rust has already increased dramatically
as a result of reading the first section. Given that the second section is
mostly what the [official book](https://doc.rust-lang.org/book/) in the Rust documentation covers, I skipped
it---for now---and went straight to section three. Traits are everywhere in
Rust, yet they don't get extensive explanation in the official book. I'm glad to
see that an entire section was dedicated to them in _Programming Rust_.

Unfortunately, some things are still skimmed over. For instance, the
relationship between trait objects and static methods is mentioned but not
elaborated upon. The book mentions the details are tedious, and implies they are
of little consequence, but it left me wondering.  Perhaps I will have a clearer
picture on this intricacy at the end of the book. Perhaps details are included
in the section I skipped over, or elaborated upon later in the book, so I'm wary
to consider this in actual criticism at this point.


## Where to go from here {#where-to-go-from-here}

I definitely plan to finish the book. Reading it is a joy. It definitely helps
if you go in with at least some Rust knowledge, because it ramps up quite
quickly. If you're at the stage where you understand what Rust does but still
struggle with the borrow checker and want to understand _why_ it does things,
the book is targeted at you.

The Humble Bundle is on for another eight days of writing. _Real World Haskell_
and _Introducing Elixir_ are two other books I've heard good things about and
keen to look into. They're also included in the bundle.  For $15, it's a no
brainer!
