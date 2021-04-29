# A Counter Signal

First we'll look at signals. Signals are what Silkenweb uses to make your app interactive. They're like normal variables where changes can be propagated to downstream signals.

Let's look at an example. First we'll clone the main [Silkenweb] repo, which contains all the examples in this tutorial:

```bash
git clone https://github.com/silkenweb/silkenweb
```

The tutorial examples can all be found in the `tutorial` subdirectory.

```rust,no_run,noplayground,ignore
{{#rustdoc_include ../counter-signal/src/main.rs:body}}
```

We only show the important parts of the code here, but you can show the full code by clicking on the "eye" icon in the top right of the code snippet.

Let's run this example:

```bash
cargo run
```

We get the output:

```text
The count is 0
Setting `count` to 1
The count is 1
Dropping `print_count`
Setting `count` to 2
```

We created a `Signal` called `count`. We map over `count` to produce another `Signal` called `print_count` which simply prints out the current value of `count`:

```rust,no_run,noplayground,ignore
{{#rustdoc_include ../counter-signal/src/main.rs:print_count}}
```

`map` is just like `Option::map`, `Iterator::map`. It maps one signal type to another by transforming the inner value.

`print_count` runs once when it is initialized, and once for every update to `count`, until `print_count` is dropped. Once a `ReadSignal` is dropped, it will no longer respond to changes in upstream `Signal`s.

We define `set_count` so we can update the counter:

```rust,no_run,noplayground,ignore
{{#rustdoc_include ../counter-signal/src/main.rs:define_set_count}}
```

We can update the counter with:

```rust,no_run,noplayground,ignore
{{#rustdoc_include ../counter-signal/src/main.rs:set_count}}
```

Next we'll look at how we use signals to make a simple interactive app.

[Silkenweb]: https://github.com/silkenweb/silkenweb
