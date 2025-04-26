# Tech Context

## Used Technologies

The different components of the project are written in Rust. This ensures a
fast startup for the GitHub application. For the command line application it
ensures minimal dependencies.

## Development setup

This project uses Nix flakes to create a reproducible development environment.

The project can be built with `nix build .#github-app`.

The Nix flake also contains a development shell with all required tools like
the Rust toolchain, Cargo and the following scripts

- `lint` to execut all linting rules
- `chkfmt` to check for correct code formatting
- `fix` to fix linting and code formatting errors

## Dependencies

This is an application, therefore dependencies versions must be pinned to a
specific version. Only dependencies that provide substantial functionality
should be used. If you only need to use a small part of a dependency prefer
writing the code yourself instead. The dependencies need to be well
maintained and used by a sizeable amount of other projects.

### Github Application

Axum is used as the REST server framework to implement the Github event
endpoints.
