# A Reactive List of Counters

In the last example, we kept the state of each counter between frames, but we were still re-rendering the list on each frame. Lets work through some code to fix that:

```rust,no_run,noplayground,ignore
{{#rustdoc_include ../reactive-counter-list/src/main.rs:main}}
```

We'll get Silkenweb to manage the list for us:

```rust,no_run,noplayground,ignore
{{#rustdoc_include ../reactive-counter-list/src/main.rs:new_list}}
```

There are different options for managing lists, depending on what sort of functionality you want. We'll use `OrderedElementList` with a `div` root element, as it will cover our needs.

We'll base the main counter on the length of the list. This will be reactive to changes in the list length because we wrapped the list in a signal. Anything that changes the list has to go through the signal, so the counter will update.

Lets implement the `push_button` function:

```rust,no_run,noplayground,ignore
{{#rustdoc_include ../reactive-counter-list/src/main.rs:push_button}}
```

Most of this is similar to what we've seen before, but the way we mutate the list is new:

```rust,no_run,noplayground,ignore
{{#rustdoc_include ../reactive-counter-list/src/main.rs:mutate_list}}
```

`mutate` lets us mutably borrow the list inside our closure, so we can apply our mutations. In this case we insert a new element into the list. Silkenweb will propagate the changes to any dependent signals: in this case, the main counter text will be updated. When we insert items, we use the current list length as the key, so new items will always go onto the end of the list.

Here's the implementation for `pop_button`:

```rust,no_run,noplayground,ignore
{{#rustdoc_include ../reactive-counter-list/src/main.rs:pop_button}}
```

It doesn't use anything new in terms of Silkenweb functionality, so it's just shown for completeness.

That covers most of the concepts in Silkenweb. For some of the features we haven't covered, check out the [Further Reading](further-reading.md) chapter.
