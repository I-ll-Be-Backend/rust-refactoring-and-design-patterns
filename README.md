# Rust Refactoring and Design Patterns

Code from the [I'll Be Backend](https://www.youtube.com/@illbebackend) videos: messy Rust code gets better, one small, safe refactoring at a time. Every lesson has a video, an [interactive version](https://illbebackend.dev) you can step through, and a folder here with the code after every step.

## Code smells

| Lesson | Steps | Video | Interactive |
|---|---|---|---|
| [Long parameter list: introduce a parameter object](long-parameter-list-introduce-a-parameter-object/) | 11 | [Watch](https://www.youtube.com/watch?v=pZ98AB5_Yuc) | [Open](https://illbebackend.dev/lessons/long-parameter-list-introduce-a-parameter-object/rust/) |
| [Clean code: replace comments with code](replace-comments-with-code/) | 13 | [Watch](https://www.youtube.com/watch?v=ktwrzeIwclc) | [Open](https://illbebackend.dev/lessons/replace-comments-with-code/rust/) |

## Principles

| Lesson | Steps | Video | Interactive |
|---|---|---|---|
| [DRY: one rule, one place](dry/) | 11 | [Watch](https://www.youtube.com/watch?v=UsVGgD1mqtI) | [Open](https://illbebackend.dev/lessons/dry/rust/) |
| [KISS: replace a rule engine with plain code](kiss-unwind-a-clever-abstraction/) | 13 | [Watch](https://www.youtube.com/watch?v=cfWPLWyF6nA) | [Open](https://illbebackend.dev/lessons/kiss-unwind-a-clever-abstraction/rust/) |
| [YAGNI: delete speculative options](yagni-delete-speculative-options/) | 13 | [Watch](https://www.youtube.com/watch?v=05fNDrl7SZM) | [Open](https://illbebackend.dev/lessons/yagni-delete-speculative-options/rust/) |

## How to use it

- Open a lesson folder and read its README, then the step folders in order.
- `sh check.sh` compiles every step in Docker with `rust:1.83-slim`, the same check every lesson passed before its video went out.
- Found a bug in a lesson? Open an issue.

## License

The code is [MIT No Attribution](LICENSE): use it however you like. The videos, the name and the logo are covered by the [terms](https://illbebackend.dev/terms/).
