# Tasks

## Accumulators

Generalize sum in terms of actions and their inverse.

## Animation

## Dom Child List

What should the primitive operations on child lists be? We should be able to implement filtering, sorting, mapping in terms of the primitives.

## Flattening Signals

Can we implement a `flatten` method on nested `ReadSignal`s?

```rust
    impl<T> ReadSignal<ReadSignal<T>> {
        fn flatten(&self) -> ReadSignal<T>;
    }
```

## Mapping Over ReadSignal Products

`(signal0, signal1).map(...)` for larger tuples. Is it easier if we have signal flattening?

## Optimizing DOM Tree Representation

Currently `Element` mirrors the real DOM tree. It only really needs to store a tree of reactive nodes and their reactive children. Is this even worth implementing?

## Empty Elements

Sometimes it's useful to have an expression/return type of `ReadSignal<Option<Element>>` to represent a node that may or may not exist in a reactive way.

- Is it best to implement this for `Option<Element>` as well?

## Template Elements

Elements should have a `.cache()` method that globally caches the node by call site and uses `clone_node` to generate the elements. Will this work for reactive elements?
