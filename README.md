# Silkenweb

Silkenweb is a reactive web library for writing single page apps

## Features

- Fine grained reactivity using signals
- No VDOM
- Uses plain Rust syntax rather than a macro DSL

## Example: A Simple Counter

```rust
use silkenweb::{
    elements::{button, div, p},
    mount,
    signal::Signal,
};

fn main() {
    let count = Signal::new(0);
    let set_count = count.write();
    let inc = move |_, _| set_count.replace(|&i| i + 1);
    let count_text = count.read().map(|i| format!("{}", i));

    let app = div()
        .child(button().on_click(inc).text("+"))
        .child(p().text(count_text));

    mount("app", app);
}
```

## Learning

- Read the tutorial: [Learning Silkenweb With Entirely Too Many Counters](https://silkenweb.netlify.app/)
- Check out the `examples` folder
