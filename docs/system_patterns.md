# System Patterns

## System Architecture

The project consists of a Github application that listens for `workflow_run`
events and acts upon them.

The second component is a command line interface application to simplify
usage of the Koritsu flow for the developer.

## Design Patterns

The interaction with Github APIs is hidden behind a facade. The facade
can be easily mocked in integration tests.
