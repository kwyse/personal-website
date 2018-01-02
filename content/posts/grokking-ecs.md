---
title: "Grokking Entity-Component-Systems"
date: 2017-12-28T23:00:59Z
draft: true
---

I've spent many commutes in the last few months learning the intricacies of
[Specs](https://github.com/slide-rs/specs), a entity-component-system (ECS)
written in Rust and, to be more broad, ECSs in general. ECSs have proved to be a
much deeper topic than I had initially anticipated. Now I'd like to explain my
findings in order to solidify that knowledge.

ECSs are a decoupling pattern. They're most frequently seen in game development
where we often have many similar yet distinct types of *game objects*. Games are
effectively giant state machines and it can be hard to create an object-oriented
hierarchy that represents this. ECSs instead implores the use of data-driven
programming, with components representing the data to be acted on, systems
acting on those components to mutate them, and entities linking components for
each *game object*.

## High level design

There isn't clear consensus on *how* one should go about building an ECS.
They're a high-level concept that implementation details can be optimised to a
particular use case. But there are clear themes, which I've included here, as
well as design decisions that I found particularly interesting.

The first revelation is that entities needn't be fat. Entities represent a game
object, like the player. You may think a *player* object must be complex,
composed of many other objects like hardware input, a hit box for collision
detection, and a sprite. Not so. Instead it can be just a unique ID.

As for these other objects that compose a player, they are components. Ideally,
components should only contain primitive types. It is vital that we are able to
retrieve the component instance for a particular component and for a particular
entity efficiently (*O(1)*), because these operations will make up most of the
game loop, as you'll see shortly. We accomplish the first part by storing each
type of component in a different collection. All positions for all entities will
be stored in one collection and all sprites will be stored in another. How the
second requirement is fulfilled depends on the underlying storage medium.

For a map data structure, it's simple because lookup for a given ID (the entity
ID) will always be amortized to constant time complexity. But maps have
overhead. For example, the hashing function must be ran on every insertion and
lookup for hash maps.

For arrays, we could insert the entity at an index that matches its ID. The
problem here is that the array must be as large as the largest entity ID. This
brings a distinction between *hot* components, which we'll likely have many of,
like entity positions, and *cold* components which we may only have a few of,
the hardware input context. In general, arrays are a better storage medium for
hot components and maps are better for cold components, though other data
structures exist and may suit your particular use case more. This binary
division may also not create enough granularity for your
use case.

Efficient lookup is vital because we will need to iterate through these
collections in our systems. We could have a *MovementSystem* that adjusts an
entity's position based on its velocity. This system must iterate through all
components in the velocities collection (probably an array because we would
expect there to be many entities that have a velocity component) and join on the
indexes that also exist in the positions collection. Ideally the API should
seamlessly expose this join, because it's generic across all systems and all
components. All the system cares about is being provided components that it
needs to act upon that belong to the same entity. This keeps the system very
small. It should only include the logic to logic to mutate a position given a
velocity.

Structuring the code this way gives a clear decoupling benefit. What may not be
as clear is the performance benefit. Remember that components should ideally
only contain primitive types, and appropriately abstracted components should be
as small as possible. This means their collections should also be small in terms
of memory. We can then take advantage of the CPU caches. If our position
component is simply a coordinate with two 64-bit floating point components, an
*x* and a *y* component, we could have as many as a few thousand position
components and still fit comfortably in the L1 cache.

## A Rust implementation with Specs

Specs relies on another crate called
[`shred`](https://github.com/slide-rs/shred), used for shared resource
dispatching. This in turn relies on a crate called
[`mopa`](https://github.com/chris-morgan/mopa). Let's start there and work our
way backwards.

`mopa`, or *My Own Personal Any*, allows you to covert an object that implements
a certain trait into the concrete object, known as downcasting. This emulates
downcasting in the [`Any`](https://github.com/chris-morgan/mopa) trait in the
Rust standard library.

`shred` uses this for storing arbitrarily-typed structs. What we were calling
components above, `shred` calls a *resource*. Its `Resource` trait is
implemented for all types that adhere to Rust's borrowing model (all those that
implement `Any + Send + Sync`), but this `Any` is `mopa`'s `Any`, not the
standard library `Any`, which means we can only downcast our own `Resource`s,
but that's all we need. You can see this in the
[`res`](https://github.com/slide-rs/shred/blob/master/src/res/mod.rs) module of
`shred`.

A neat optimisation is that `Resource`s are stored in a `FnvHashMap`. This uses
the *FNV* hashing algorithm instead of the default *SipHash* algorithm. The
former is faster when using smaller keys, but is less secure. This is
perfectly acceptable in this instance because our keys are just unsigned
integers (wrapped in `std::any::TypeId`, itself wrapped in `shred`'s
`ResourceId`). Benchmarks can be found
[here](http://cglab.ca/~abeinges/blah/hash-rs/).

`shred` revolves around its `Fetch` and `FetchMut` structs. These are
effectively wrappers for `Ref` and `RefMut` from the standard library,
respectively. When we want to read a component, we specify a system with a
`Fetch` of that same type. We do the same for components we want to modify, but
use `FetchMut` for those instead.

A really ergonomic feature of this API is that you declare the components a
system corresponds to with a tuple. This allows you to include as many read or
write resources in a system as you want... almost. There's a [`hard
limit`](https://github.com/slide-rs/shred/blob/master/src/system.rs#L215) of 26,
though systems should never reach close to that number in practice.

That's the crux of how `shred` is working under the hood. Check the project's
[README](https://github.com/slide-rs/shred/blob/master/README.md) for an example
usage.

Specs fine tunes this model specifically for ECSs. Its API uses terminology
that's more familiar. All structs that our systems want to work on my implement
the `Component` trait. The tuple that defines the components our systems work on
accepts `ReadStorage` and `WriteStorage` types instead of `Fetch` and
`FetchMut`. It also introduces different storage strategies like `VecStorage`
and `HashMapStorage`, with the same nuances described in the previous section.

# Demonstration

Benchmarks, include profiles of cache misses
