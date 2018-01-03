---
title: "Grokking Entity-Component-Systems"
date: 2018-01-03T21:57:06Z
---

I've spent many commutes in the last few months learning the intricacies of
[Specs](https://github.com/slide-rs/specs), a entity-component-system (ECS)
written in Rust and, to be more broad, ECSs in general. ECSs have proved to be a
much deeper topic than I had initially anticipated. Now I'd like to explain my
findings in order to solidify that knowledge.

ECSs are a decoupling pattern. They're most frequently seen in game development
where we often have many similar yet distinct types of *game objects*. Games are
effectively giant state machines and it can be hard to create an object-oriented
hierarchy that represents this. ECSs instead implore the use of data-driven
programming, with components representing the data to be acted on, systems
acting on those components to mutate them, and entities linking components for
each game object.

## High level design

There isn't clear consensus on *how* one should go about building an ECS.
They're enough of a high-level concept that implementation details can be
optimised to a particular use case. But there are clear themes, which I've
included here, as well as design decisions that I found particularly
interesting.

The first revelation is that entities needn't be fat. Entities represent a game
object, like the player. You may think a *player* object must be complex,
composed of many other objects like hardware input, a hit box for collision
detection, and a sprite. Not so. Instead it can be just a unique ID.

As for these other objects that compose a player, they are components. Ideally,
components should only contain primitive types. It is vital that we are able to
retrieve the component instance for a particular component and for a particular
entity efficiently (*O(1)*), because these operations will make up most of the
game loop, as you'll see shortly. We accomplish the first part by storing each
type of component in a different collection. For example, all positions for all
entities will be stored in one collection and all sprites will be stored in
another. How the second requirement is fulfilled depends on the underlying
storage medium.

For a map data structure, it's simple because lookup for a given ID (the entity
ID) will always be amortized to constant time complexity. But maps have
overhead. For example, the hashing function must be ran on every insertion and
lookup for hash maps.

For arrays, we could insert the entity at an index that matches its ID. The
problem here is that the array must be as large as the largest entity ID. This
brings a distinction between *hot* components, which we'll likely have many of,
like entity positions, and *cold* components which we may only have a few of,
like the keyboard input context. In general, arrays are a better storage medium
for hot components and maps are better for cold components, though other data
structures exist and may suit your particular use case more. This binary
division may also not create enough granularity for your use case.

Efficient lookup is vital because we will need to iterate through these
collections in our systems. We could have a *MovementSystem* that adjusts an
entity's position based on its velocity. This system must iterate through all
components in the velocities collection (probably an array because we would
expect there to be many entities that have a velocity component) and join on the
indexes that also exist in the positions collection. Ideally the API should
seamlessly expose this join, because it's generic across all systems and all
components. All the system cares about is being provided components that it
needs to act upon that belong to the same entity. This keeps the system small.
It should only include the logic to mutate a position given a velocity.

Structuring the code this way gives a clear decoupling benefit. What may not be
as clear is the performance benefit. Remember that components should ideally
only contain primitive types, and appropriately abstracted components should be
as small as possible. This means their collections should also be small in terms
of memory. We can then take advantage of the CPU caches. If our position
component is simply a coordinate with two 64-bit floating point components, an
*x* component and a *y* component, we could have as many as a few thousand
position components and still fit comfortably in the L1 cache, not to mention
the L2 and L3 caches for more realistic collection sizes.

## A Rust implementation with Specs

Specs relies on another crate called
[`shred`](https://github.com/slide-rs/shred), used for shared resource
dispatching. This in turn relies on a crate called
[`mopa`](https://github.com/chris-morgan/mopa). Let's start there and work our
way backwards.

`mopa`, or *My Own Personal Any*, allows you to covert an object that implements
a certain trait into the concrete object, known as downcasting. This emulates
downcasting on the [`Any`](https://doc.rust-lang.org/std/any/trait.Any.html)
trait in the Rust standard library.

`shred` uses this for storing arbitrarily-typed structs. What we were calling
a component above, `shred` calls a *resource*. Its `Resource` trait is
implemented for all types that adhere to Rust's borrowing model, all those that
implement `Any + Send + Sync`, but this `Any` is `mopa`'s `Any`, not the
standard library `Any`, which means we can only downcast our own `Resource`s,
but that's all we need. You can see this in the
[`res`](https://github.com/slide-rs/shred/blob/master/src/res/mod.rs) module of
`shred`.

A neat optimisation is that `Resource`s are stored in a `FnvHashMap`. This uses
the *FNV* hashing algorithm instead of the default *SipHash* algorithm. The
former is faster when using smaller keys, but is less secure. This is
perfectly acceptable in this instance because our keys are just unsigned
integers (wrapped in
[`std::any::TypeId`](https://doc.rust-lang.org/std/any/struct.TypeId.html),
itself wrapped in `shred`'s `ResourceId`). Benchmarks can be found
[here](http://cglab.ca/~abeinges/blah/hash-rs/).

`shred` revolves around its `Fetch` and `FetchMut` structs. These are
effectively wrappers for
[`Ref`](https://doc.rust-lang.org/std/cell/struct.Ref.html) and
[`RefMut`](https://doc.rust-lang.org/std/cell/struct.RefMut.html) from the
standard library, respectively. `Ref` and `RefMut` are in turn the wrappers for
objects contained within a
[`RefCell`](https://doc.rust-lang.org/std/cell/struct.RefCell.html) when it is
*borrowed*.

`RefCell`s are used when we want to enforce Rust's borrowing rules at runtime
rather than compile time. These rules, at their core, are that we can only have
one mutable reference to an object at a time, or multiple immutable references
to it. As such, we can have only have one `FetchMut` reference to a resource at
a time, or mutiple `Fetch` ones. When we want to read a component, we specify a
system with a `Fetch` of that same type. We do the same for components we want
to modify, but use `FetchMut` for those instead.

A really ergonomic feature of this API is that you declare the components a
system corresponds to with a tuple. This allows you to include as many read or
write resources in a system as you want... almost. There's a [`hard
limit`](https://github.com/slide-rs/shred/blob/master/src/system.rs#L215) of 26,
though systems should never reach close to that number in practice.

That's the crux of how `shred` is working under the hood. Check the project's
[README](https://github.com/slide-rs/shred/blob/master/README.md) for example
usage.

Specs fine tunes this model specifically for ECSs. Its API uses terminology
that's more familiar. All structs that our systems want to work on must
implement the `Component` trait. The tuple that defines the components our
systems work on accepts `ReadStorage` and `WriteStorage` types instead of
`Fetch` and `FetchMut`. It also introduces different storage strategies like
`VecStorage` and `HashMapStorage`, with the same nuances described in the
previous section.

# Demonstration

Theory only goes so far. We want a result. Let's follow along with the examples
above.  We'll create an ECS that modifies an entity's position according to it's
velocity. Rather than just show numbers being affected, let's actually show the
entity moving across the screen. We'll use SDL2 for events, rendering and window
management.

The following application was built with these crates:

* sdl2 (0.31.0)
* specs (0.10.0)
* specs-derive (0.1.0)

First we declare out components. This includes a sprite component that wraps
SDL2's `Rect` struct. SDL2 makes it easy to render `Rect`s to screen.

```rust
#[derive(Component)]
struct Position {
    x: f64,
    y: f64,
}

#[derive(Component)]
struct Velocity {
    x: f64,
    y: f64,
}


#[derive(Component)]
struct Sprite(Rect);
```

Then we declare out systems. The first one is to update the position of an
entity given its velocity.

```rust
struct MovementSystem;

impl<'a> System<'a> for MovementSystem {
    type SystemData = (
        Fetch<'a, Duration>,
        ReadStorage<'a, Velocity>,
        WriteStorage<'a, Position>
    );

    fn run(&mut self, data: Self::SystemData) {
        let (dt, velocities, mut positions) = data;

        for (vel, pos) in (&velocities, &mut positions).join() {
            pos.x += vel.x * dt.subsec_nanos() as f64 / 1_000_000_000.0;
            pos.y += vel.y * dt.subsec_nanos() as f64 / 1_000_000_000.0;
        }
    }
}
```

This matches the logic described prior. The only difference is that we also
include a *delta time* input value. This represents the amount of time that has
passed from one frame to the next. We need this because we don't have control on
exactly when our function will be called again. We can aim for a target, say, 60
times per second, but we'll never hit that exactly. It may only be a few
milliseconds off here and there, but that adds up the longer the game is
running. Pretty quickly we would have vastly inaccurate positions if you don't
scale them like this! [Integration
Basics](https://gafferongames.com/post/integration_basics/) by Glenn Fiedler
explains why this happens.

The other system we need converts logical world coordinates to screen
coordinates.

```rust
struct RenderSystem;

impl<'a> System<'a> for RenderSystem {
    type SystemData = (ReadStorage<'a, Position>, WriteStorage<'a, Sprite>);

    fn run(&mut self, data: Self::SystemData) {
        let (positions, mut sprites) = data;

        for (pos, sprite) in (&positions, &mut sprites).join() {
            sprite.0.set_x((pos.x * PIXELS_PER_UNIT) as i32);
            sprite.0.set_y((pos.y * PIXELS_PER_UNIT) as i32);
        }
    }
}
```

Using logical world units for an entity's position frees us from the details of
our rendering process. When it comes to rendering, we simply scale the position
by a constant factor to get screen coordinates, which is used by our sprite for
rendering.

Almost there. We now need to hook this all up to the `World`, which manages the
entities.

```rust
let mut world = World::new();
world.add_resource(Duration::new(0, 0));
world.register::<Position>();
world.register::<Velocity>();
world.register::<Sprite>();

let initial_pos = Position { x: 2.0, y: 2.0 };
let initial_vel = Velocity { x: 1.0, y: 0.0 };
let sprite = Sprite(Rect::new(
        (initial_pos.x * PIXELS_PER_UNIT) as i32,
        (initial_pos.y * PIXELS_PER_UNIT) as i32,
        32,
        32
));
world.create_entity()
    .with(initial_pos)
    .with(initial_vel)
    .with(sprite)
    .build();

let mut dispatcher = DispatcherBuilder::new()
    .add(MovementSystem, "movement_system", &[])
    .add(RenderSystem, "render_system", &["movement_system"])
    .build();
```

It's then just one line to update all of our entities.

```rust
dispatcher.dispatch(&mut world.res);
```

Of course, we need some additional infrastructure around this. The above line
should belong in the application run loop. That loop should also contain input
handling and rendering to a hardware context.

You may be able to implement those as systems as well, but at some point you
will hit a boundary where the objects are too large. This will often be with
input and output. Rendering to screen is a complex process, and should probably
be done outside of the ECS. This demonstrates that ECSs are not appropriate for
the entire application, particularly on the boundaries, but still very useful
for internal logic that we have full control over.

If you would like to learn the details of run loops, check out [Fix Your
Timestep!](https://gafferongames.com/post/fix_your_timestep/) It's probably the
most quoted article on the subject and does a fine job explaining the various
approaches.

## Results

[Gist of the source code](https://gist.github.com/kwyse/1d6be3de1c95d05502e10b6dba3cc6be)

The above includes the simplest kind of run loop with a fixed timestep of 1/60th of a
second. The results are hopefully a white square moving across a black abyss.

{{% center %}}
![Grokking ECS results](/images/grokking_ecs_result.gif)
{{% /center %}}

There are many ways to improve this. You could use a more sophisticated run loop
that can handle variable time steps. Or you could use the parellel iterators
offered by Specs to improve performance. It's probably a good idea to better
define the boundaries of our ECS explcitely as well. Modularise all of that and
you have the beginnings of a game!
