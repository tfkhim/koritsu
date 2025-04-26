# Koritsu Workflow Utilities

This repository contains utilities to simplify the
[Koritsu](https://debitoor.com/blog/trunk-based-development-how-we-fixed-it-with-koritsu)
workflow.

The project documentation can be found in the `docs` folder.

## Development

Please read the `docs/development_guidelines.md` file. It describes conventions
and best practices of this project.

### Building the project

To build the project you can run

    cargo build

If you want to build the Nix package you can run

    nix build .#github-app

### Tests

To run the test suite execute the following command

    cargo test

### Linting and formatting

To run the linting rules use the command

    lint

Check correct code formatting with

    chkfmt

Fix code formatting errors

    fix
