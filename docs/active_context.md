# Active Context

## Current Focus

Implementing the GitHub application to receive and process `workflow_run` events.

## Recent Changes

- Replaced the `/hello` endpoint with `/github/events`.
- Implemented deserialization for `workflow_run` event payloads.
- Added basic logging of workflow name and status.
- Added integration tests for the new endpoint.
- Added `serde` and `serde_json` dependencies.

## Next Steps

- Continue implementing the core GitHub application logic, including checking CI status and merging branches based on the Koritsu workflow.
- Implement the GitHub API integration facade.
- Develop the Command Line Interface (CLI) tool.

## Active Decisions & Considerations

- Ensuring accurate deserialization of GitHub `workflow_run` payloads.
- Designing the logic for checking CI status and performing merges.

## Important Patterns & Preferences

- Adherence to the Koritsu workflow principles.
- Use of Rust and Nix flakes as defined in `tech_context.md`.
- Application of the Facade pattern for GitHub API interactions as noted in `system_patterns.md`.

## Learnings & Insights

- The project is progressing with the implementation of core event handling.
- Need to carefully consider the structure of GitHub API interactions and the merging logic.
