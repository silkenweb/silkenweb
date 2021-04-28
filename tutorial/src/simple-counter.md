# A Simple Counter

Our first app will be a very simple counter. The interactive equivalent of "Hello, world!". First we'll clone the main [Silkenweb] repo, which contains all the examples in this tutorial:

```bash
git clone https://github.com/silkenweb/silkenweb
```

The tutorial examples can all be found in the `tutorial` subdirectory. Lets run the first example so we can see how the code translates:

```bash
cd silkenweb/tutorial/counter
trunk serve --open
```

This should open a browser with a simple counter app. Let's have a look at the code:

```rust,no_run,noplayground
{{#rustdoc_include ../counter/src/main.rs:body}}
```

We only show the important parts of the code here, but you can show the full code by clicking on the "eye" icon in the top right of the code snippet.

This code defines a counter app and `mount`s it on the page.

Next we'll define the actual counter app:

```rust,no_run,noplayground
{{#rustdoc_include ../common/src/lib.rs:body}}
```

We use the [builder pattern] to define elements of our app. In the `define_counter` function, you can see that we define a `div` element with two child `button` elements, and some text. The text is a `ReadSignal<String>`, which means it will vary over time when we write to `count`.

We define a button with `define_button`. It adds a callback to handle mouse clicks which updates the count signal.

That's it for our simple button. Next we'll look at a slightly more complex example.

[Silkenweb]: https://github.com/silkenweb/silkenweb
[builder pattern]: https://en.wikipedia.org/wiki/Builder_pattern
