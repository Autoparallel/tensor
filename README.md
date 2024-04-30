![Github Actions](https://github.com/primitivefinance/arbiter/workflows/test/badge.svg)
[![Twitter Badge](https://badgen.net/badge/icon/twitter?icon=twitter&label)](https://twitter.com/autoparallel)

# extensor
Tensors are a prolific mathematical object that also appear in computation quite often. 
This repo serves as a place to build tensors and related structures such as Clifford algebras in Rust.

Goals are:
- **type-safe**: Mathematically sound and true to form.
- `no_std` support.
- **no** (or very few) **external dependencies**.
- **parallelizable** with support for open source GPU libs.
- **extensibile**: With this crate set up, we allow the end user to bring structures and definitions into their own projects so that more can be added to their own tensors as need be.

## WIP
This repository is very new and subject to some large changes. 
The layout of the workspace is not yet finalized and some items will be removed to make this as easy to work with as possible.
Expect a name change down the road to match the crates release.