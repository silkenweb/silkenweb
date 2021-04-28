# A Reactive List of Counters

In the last example, we kept the state of each counter between frames, but we were still re-rendering the list on each frame. Lets fix that:

```rust,no_run,noplayground,ignore
{{#rustdoc_include ../reactive-counter-list/src/main.rs:main}}
```

We'll get Silkenweb to manage the list for us:

```rust,no_run,noplayground,ignore
{{#rustdoc_include ../reactive-counter-list/src/main.rs:new_list}}
```

There are different options depending on what sort of list you want, but we'll use `OrderedElementList` with a `div` root element.

We'll base the main counter on the length of the list. Since we wrapped the list in a signal, this will be reactive to changes in the list length. Anything that changes the list has to go through the signal, so the counter will update.

Lets implement the `push_button` function:

```rust,no_run,noplayground,ignore
{{#rustdoc_include ../reactive-counter-list/src/main.rs:push_button}}
```

Most of this is straightforward now, but the way we mutate the list is new:

```rust,no_run,noplayground,ignore
{{#rustdoc_include ../reactive-counter-list/src/main.rs:mutate_list}}
```

`mutate` lets us mutably borrow the list inside our function, so we can apply our mutations. In this case we insert a new element into the list. Silkenweb will propagate the changes to any dependent signals: in this case, the main counter text will be updated. We used the current list length as the new list item key.

Here's the implementation for `pop_button`:

```rust,no_run,noplayground,ignore
{{#rustdoc_include ../reactive-counter-list/src/main.rs:pop_button}}
```

It doesn't use anything new in terms of Silkenweb.

That covers most of the concepts in Silkenweb. Check out the [Further Reading](further-reading.md) chapter for some of the features we haven't covered.
