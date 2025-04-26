# Development guidelines

This project uses the follwing conventions and guidelines for development. You
should follow them as close as possible.

## Quality

- Write tests for new code
  - Prefer integration style test
  - For code that can not be tested with an integration test you should write a
    unit test in the module instead
- After changing the codebase always execute the following quality assurance
  steps and fix potential errors:
  - Compile the code
  - Run the test suite
  - Fix potential issues with `fix`
  - Run formatting checks with `chkfmt`
- Ignoring linter errors should always be a last resort. Add a comment why
  ignoring the error is better than fixing it
- Avoid comments that describe trivial things or only reformulate what the
  code is doing. Instead think about what might not be obvious for someone
  reading the code the first time and describe why the code is how it is.
- Make changes self-contained and focused. Changes that are improvements but
  are not strictly necessary for the current task should be their own commit.

## Documentation

- Update and extend the documentation in `/docs`
- Keep `README.md` in sync with new capabilities

## Commits

- Use conventional commit messages
- Do not use an upper case letter after the commit type
