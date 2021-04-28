# A List of Memoized Counters

In the last example, we wanted to keep the state of the existing counters when we change the number of counters. This example will show one way to do that: [memoization].

Lets dive straight into the code:

```rust,no_run,noplayground,ignore
{{#rustdoc_include ../memoized-counter-list/src/main.rs:main}}
```

This looks very similar to the previous example. The main difference is the new `counter_elem_cache`:

```rust,no_run,noplayground,ignore
{{#rustdoc_include ../memoized-counter-list/src/main.rs:create_cache}}
```

This creates a cache scoped using the normal rust scoping rules. In this case it will be captured by the closure. The expiry policy is to cache an item until the next *frame*, and discard the item if it wasn't used. To do this we need to tell it about *frame* scoping:

```rust,no_run,noplayground,ignore
{{#rustdoc_include ../memoized-counter-list/src/main.rs:create_frame}}
```

Now we need to implement `define_counter_list`:

```rust,no_run,noplayground,ignore
{{#rustdoc_include ../memoized-counter-list/src/main.rs:define_counter_list}}
```

Looking inside, we can see where it uses the cache:

```rust,no_run,noplayground,ignore
{{#rustdoc_include ../memoized-counter-list/src/main.rs:get_cached_counter}}
```

This will reuse counters between frames when we update the main counter. The individual counters will not be re-rendered unless they change. Unfortunately, the list of counters gets reconstructed each frame. We'll address that in the next example.

[memoization]:https://en.wikipedia.org/wiki/Memoization
