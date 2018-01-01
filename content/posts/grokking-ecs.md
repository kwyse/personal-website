---
title: "Grokking Entity-Component-Systems"
date: 2017-12-28T23:00:59Z
draft: true
---

For the last few months I've spent many commutes learning the intracacies of
Specs, a entity-component-system (ECS) written in Rust and, to be more broad,
ECSs in general. I'm now at a point I feel like I could try explaining my
findings to help solidify my knowledge.

Let's begin with what an ECS is and what problems they are designs to solve.
ECSs are essentially a decoupling pattern. They're most frequently seen in game
development where often have many similar yet distinct types of game objects. It
can be hard to create an object-oriented hierarchy that represents this. ECSs
would call these game objects *entities*, but implementation-wise they are often
no more then an ID.

What we have instead is a collection of components that they game objects have
in common. Take position, for example, which should be common to a great deal of
*game objects*. We would store the position components in their own collection.
A key requirement is that we can index them efficiently, because that index is
the entity ID that the component belongs to. So say we have two collections, one
for positions, and one for velocities. We iterate through both of the
collections and join on the indexes. If a component exists in both collections,
it means those components belong to the same entity.

What mutates those components? That's the job of systems. Systems will often do
that iteration, or at least be called with each component collection for a given
index. In the above example, it would make sense to have a *movement system*
that has read access to velocities and write access to positions. It would
mutate the position by the velocity for each index that they share.

That sounds like a very frequent operation if we have a lot of entities with
positions, don't you think? That's another advantage of ECSs. We can structure
this iteration so that data access happens in batches. Because these components
are often small (perhaps just two 64-bit floats in the case of a position
component), they can leverage data locality. The entire collection will often be
able to take advantage of the CPU caches, resulting in very fast data access.

Contrast this with an object-oriented apprach. Often you will run into the
diamond problem for various game object hierarchies. For example, 
