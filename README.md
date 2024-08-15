# Yew.rs + Nix + Docker template

A basic Yew.rs project, with Nix and Docker integration

## Usage

Change the project name in `Cargo.toml`, and the description in `flake.nix`.

```sh
trunk serve --open # start a development server at 127.0.0.1:8080 with auto reloading and open it in your browser
trunk build --release # build the project into the `dist/` directory

# NOTE:
cargo run # will panic, use the `trunk` CLI
```

If you aren't using the Nix dev shell, you might need to:

- Add the `wasm32-unknown-unknown` Rust target: `rustup target add wasm32-unknown-unknown`
- [Install Trunk](https://trunkrs.dev/#install)

## Nix

This project contains a Nix flake, with Crane to build the Rust project.
It is based on [this example](https://crane.dev/examples/trunk-workspace.html).
It builds the project and exposes HTML + CSS + JS + WASM files in the output package.
It also contains a development shell with all needed packages.
When running `nix run` it will build the package and serve it with darkhttpd on `0.0.0.0:8080`.

## Docker

This project contains a `Dockerfile`, and a `compose.yaml` file.
The `Dockerfile` builds the project (with layer caching support) and copies the output files to a basic Nginx container.
The `compose.yaml` file builds the image and exposes the website at `0.0.0.0:8080`.

## License

You can base your project on this template. It is licensed under the MIT (see `LICENSE`).
