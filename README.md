# Tacit

Implicit Equations to Meshes

[![Build Status](https://travis-ci.org/SallySoul/tacit.svg?branch=master)](https://travis-ci.org/SallySoul/tacit)

## Introduction

This repo contains several interconnected projects based around taking implicit
equations and generating meshes that approximate their solution.

This project was initially developed as several indipendent repos, but I
the overhead was high, and I'm migrated to a new github account, so I took the
opportunity to move all the projects into one repo.

For now there are two primary ways to use the software. The `implicit-cli` tool
can interactivley generate geometry files that are viewable via the `asap` plotter.

I am also working on a web application in the `web-client` crate.

## Design Decisions

### WebGl 1

Until Webkit supports webGL 2, I will not move towards supporting it. (Unless it becomes it
becomes an overwhelming technical limitation).

## For Developers

### Dependencies

Only the `web-client` crate has special dependencies at the moment. We use 
`wasm-bindgen`, version `0.2.33` for now. I would suggest the following.

```
cargo install wasm-bindgen-cli --version 0.2.33
```

Note that running `web-client/build.sh` will ensure you have the right version installed.
I would like to figure out how to setup a virtual env kind of setup for rust / cargo but have
not done so yet.

In order to build the `release` build of the the `web-client` you will need to have installed
the `wasm-opt` tool from the [binaryen](https://github.com/WebAssembly/binaryen) toolchain.

### Formatting

I use the defaults for `rustfmt`. The one exception I've made so far is when declaring
all elements in a matrix. In that case, preficing the delcaration with
`#[cfg_attr(rustfmt, rustfmt_skip)]` is fine. In order to format all the code in the
workspace you can run the following command.

```
cargo fmt --all
```

### Testing

In order to run all tests in the works space please the following command.

```
cargo test --all
```

The `travis-ci` job is currently configured to test the following:

*  Build and test the workspace on stable, beta, and nightly rust toolchains.
* `cargo fmt --all -- --check` must be successful on stable rust.
* `web-client/build.sh` must run succesfully on stable rust.

I would like to add release build testing of the web-client, but have not yet
added support for building `wasm-opt` into the works.
