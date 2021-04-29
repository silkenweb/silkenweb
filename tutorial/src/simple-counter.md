# A Simple Counter

Our first app will be a very simple counter. The interactive equivalent of "Hello, world!". Lets run it so we can see how the code translates:

```bash
cd tutorial/counter
trunk serve --open
```

This should open a browser with a simple counter app. Let's have a look at the code:

```rust,no_run,noplayground,ignore
{{#rustdoc_include ../counter/src/main.rs:body}}
```

This code defines a counter app and mounts it on the page using the `mount` function.

Next we'll define the actual counter app. We do this in a library crate under `tutorial/common`, as we'll reuse the counter in more complex examples:

```rust,no_run,noplayground,ignore
{{#rustdoc_include ../common/src/lib.rs:define_counter}}
```

We use the [builder pattern] to define elements of our app. In the `define_counter` function, you can see that we define a `div` element with two child buttons, and some text. `count_text` is a `ReadSignal<String>` derived from `count`, which means it will vary over time when we write to `count`.

Next we define a button with `define_button`. It adds a callback to handle mouse clicks which updates the count signal:

```rust,no_run,noplayground,ignore
{{#rustdoc_include ../common/src/lib.rs:define_button}}
```

That's it for our simple button. Next we'll look at a slightly more complex example.

[builder pattern]: https://en.wikipedia.org/wiki/Builder_pattern
