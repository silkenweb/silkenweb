# A List of Counters

Now we'll create a slightly more complex app, reusing our `define_counter` function from the previous example. First run the example so you can visualize how it relates to the code:

```bash
cd tutorial/counter-list
trunk serve --open
```

As you can see, its a simple counter that renders a list of counters corresponding to the first counter's value. Lets take a look at the code:

```rust,no_run,noplayground,ignore
{{#rustdoc_include ../counter-list/src/main.rs:main}}
```

We can see that app is structured as a counter followed by `counter_list`. Taking a closer look at `counter_list`:

```rust,no_run,noplayground,ignore
{{#rustdoc_include ../counter-list/src/main.rs:counter_list}}
```

We see that it's a signal that depends on `counter`, and generates a counter list.

`define_counter_list` is just a simple loop that builds a list of counters:

```rust,no_run,noplayground,ignore
{{#rustdoc_include ../counter-list/src/main.rs:define_counter_list}}
```

If you play around with the UI, you'll quickly find that it has one particularly annoying feature. When you change the number of counters, it will reset the state of all the other counters that you so carefully set. In the next example, we'll fix that.
