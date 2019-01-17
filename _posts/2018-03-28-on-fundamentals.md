---
layout: post
title: "On Fundamentals"
date: 2018-03-28T22:01:11+01:00
categories: book
---

A recent vacation gave me time to catch up on my backlog of books. [_Stop
Guessing_](http://www.stopguessingbook.com/), by Nat Greene, was my first
target.  It's a book about the tendency to guess solutions when faced with hard
problems, and instead proposes a rigorous, fact-backed and systematic approach
to problem solving. There is a chapter dedicated to understanding the
fundamentals of the problem space that was particularly poignant to me.

If we define a hard problem as simply something that is resistant to guesses,
impactful and yet hard to isolate and reproduce, then they're not uncommon in
the life of a software developer. But computers are deterministic. They
shouldn't possess these qualities, and indeed they don't, but software does. I
think one reason for this is the easiness of adopting a given abstraction.

Abstraction is a pillar of computer science. You're taught it from your first
foray into the field, and many of the mechanisms that feel natural to us support
it, from dependency managers to the humble function. It's necessary because
cognitive load without it would simply be too great. But with this
proliferation, where is the guarantee that we're using _good_ abstractions?

You need to have trust that you are. You trust that a vendor package, a piece of
hardware, or a third-party service doesn't have a security vulnerability, or
illicit data collection, or even just poor performance. You do this because it's
impractical to build a product from first principles.  By giving up control,
you're getting encapsulated units of value in return. It's a compromise that the
modern world is built on, but I'm not sure that it's always fully appreciated.

By losing that control, you relinquish understanding of the underlying system.
You make decisions based on inferred facts rather than reality. To bring order,
out come dogmatic processes on coding style, design patterns, architectural
patterns, usage practices; all themselves further abstractions on underlying
principles that are effectively static.

# Trends as a solar system

The JavaScript ecosystem is known for framework proliferation. It's often noted
how hard it is to keep up with the latest and greatest, but I wonder how much of
this is self-imposed.

Imagine a star with two planets orbiting it. Most of us in industry reside on
the outer planet. We work with higher level abstractions, and we travel great
distances to keep up with the latest trends.  These trends come back around, but
our solar cycle is so long that we struggle to remember the lesson learnt from
old cycles, and we must relearn.

The inner planet is the residence of lower-level abstractions. They too have a
cycle, but a much shorter one, and can build on their knowledge because their
solar cycle is much less.

And then you have the star, representing the underlying mathematics, eternal and
everlasting. This state changes very slowly, only when new fundamentals are
discovered, but we don't lose knowledge. The star only ever grows.

# To build, or to understand

There is a clear dilemma here. If a solution is sufficiently abstracted to
remove surface-level problems, and is easy enough to integrate, how can it make
business sense to forgo that solution and build another one from first
principles?

In many (all?) situations, it can't, but we don't live in this two-planet
system. We have the freedom to choose the level of abstraction that we feel is
appropriate.

Instead of always grabbing for the quick solution, I think it's vital to
understand what is lost. On one extreme, we have a perfectly-packaged solution
that precisely meets out needs, _at the current time_. Maintenance and
extensibility is another story. At the other extreme, we have a solution from
first principles, probably too expensive beyond measure and one that wouldn't be
completed within our lifetimes, but one that everyone could understand given
they all had knowledge of the fundamentals, the pieces that are common to all
because they're so simple.

Think carefully about where you want your project to sit on that spectrum.

# Learn, then understand, then build

Whilst the ideal may be impossible to achieve, I think striving for it is still
worth while. Thinking of it in terms of "relative" fundamentals, or
de-abstracting insofar as practical, may be helpful. By educating yourself on
the fundamentals, you're in a better position to make decisions for the layers
on top of it. Your design will take advantage of the underlying processes.

This will be time-consuming. The abstractions are there for a reason. But
mastery of any field requires a deep understanding of its building blocks. And
because of the way computer science has developed over the decades, resources
are numerous.

I recently acquired a copy of [_The Art of Computer
Programming_](https://www-cs-faculty.stanford.edu/~knuth/taocp.html), a book on
classical computer science. It still sells well to this day, despite being first
published in the 1960s! I think that's a testament to the fact that the
fundamentals are timeless, and that they're worth learning. I'm taking my time
with it, reading it alongside complimentary material. Regardless of your chosen
passion, you can always obtain a deeper understanding of its component parts.
And so far for me, doing that has been a real pleasure.
