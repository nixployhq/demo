# Nixploy × Topcoat

A small Rust/Topcoat web application for trying Git-based Nixploy deployments.
The home page displays the Git revision used to build it.

## Run locally

```sh
cargo run
# Or build the reproducible Nix package:
nix build
./result/bin/nixploy-demo
```

Open http://localhost:3000. Set `HOST=0.0.0.0` to listen on all interfaces;
`PORT` defaults to `3000`.

## Try an update

Edit the version text in `src/main.rs`, commit, and push to `main`.
Nixploy polls the repository, builds the committed flake, and restarts the app.
Refresh the page after the build to see the version and deployed commit change.

The app has no infrastructure dependencies. Configure its HTTPS Git repository,
`main` branch, `default` package, and `nixploy-demo` executable in your NixOS
Nixploy configuration.

MIT licensed.
