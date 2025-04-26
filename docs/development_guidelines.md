# Development guidelines

This project uses the follwing conventions and guidelines for development. You
should follow them as close as possible.

## Quality

- Write integration tests for new code
- For code that can not be tested with an integration test you should write a
  unit test instead
- Ignoring linter errors should be a last resort. Add a comment why ignoring
  the error is better than fixing it
- Avoid comments that describe trivial things or only reformulate what the
  code is doing. Instead think about what might not be obvious for someone
  reading the code the first time and describe why the code is how it is.

## Documentation

- Update and extend the documentation in /docs
- Keep README.md in sync with new capabilities

## Commits

- Make commits self-contained and focused. Changes that are improvements but
  are not strictly necessary for the current task should be their own commit.
- Make sure to run all of the following quality assurance measurements before
  committing:
  - Formatting checks
  - Linter checks
  - Unit tests
  - Integration tests
- Use conventional commit messages
- Do not use an upper case letter after the commit type
