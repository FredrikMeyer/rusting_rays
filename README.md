# A simple ray tracer

I'm making a ray tracer for fun.

At the moment I'm closely following the blog tutorials from [Brook Heisler's blog](https://bheisler.github.io/post/writing-raytracer-in-rust-part-1/).

## Run from command line

Run the example from the root directory with `cargo run`. This produces a file `test.png`.

### Build

If it is a long time since build, run `rustup update` to update deps. Then run `cargo build`.

## Run in browser

Run `npm run serve`. This uses WebAssembly to do the ray tracing in the browser. Open your browser on `localhost:8080`.

## Run/fix prettier

Check:

```
npm run prettier -- --check .
```

Write:

```
npm run prettier -- --write .
```

## Todos

Sett opp litt GH actions, a la https://dev.to/bampeers/rust-ci-with-github-actions-1ne9

## Leseliste

https://www.scratchapixel.com/lessons/3d-basic-rendering/ray-tracing-polygon-mesh/polygon-to-triangle-mesh.html
