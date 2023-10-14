# Server Side Rendering Example

This example contains various crates that render the same app using different strategies. The [app] package contains the actual app.

## Pre Rendered HTML with Client Side Hydration

This shows how to render web pages at compile time and hydrate them on first page load.

### Packages

- [xtask]: Based on the [xtask concept], this builds the app, render the initial pages, then serves the app and static pages.
- [pre-rendered-client]: A shim to run [app].

### Run

```bash
cargo xtask serve
```

Then point your browser at <http://127.0.0.1:8000/>.

## Client Side Rendering Only

### Packages

- [client-side-rendered]

### Run

```bash
cd packages/client-side-rendered
trunk serve --open
```

## Server Side Rending on Demand with Axum

### Packages

- [axum-client]
- [axum-server]

### Run

```bash
cd packages/axum-client
wasm-pack build --target=web 
cd ../axum-server
cargo run
```

Then point your browser at <http://127.0.0.1:8080/>

[app]: packages/app
[axum-client]: packages/axum-client
[axum-server]: packages/axum-server
[client-side-rendered]: packages/client-side-rendered
[pre-rendered-client]: packages/pre-rendered-client
[xtask]: packages/xtask
[xtask concept]: https://github.com/matklad/cargo-xtask/
