# TODO

## Vision

"A language empowering someone to build reliable and efficient software."

- small language with nimble feel
- lightweight typing
- built by induction to always be auto-testable
- total determinism and purity
- instant feedback
- fast interpreter for development, compile to WASM or C with no dependencies for production

## Roadmap

### Foundations

- ide + language server loop
  - get a consistent snapshot of the whole project
    - always have data from the previous iteration near the current one
  - parse (everything possibly changed)
  - compile
  - test
- strong testing infrastructure to build upon

### Language features

- first steps:
  - booleans, integers, floating point
  - if, loop
  - functions
  - packages, modules, visibility control
- ability to fuzz components in isolation
- ability to compose separately fuzzed components
- testing:
  - general way to specify properties, superset of testing and REPL
  - text encoding of corpus to be able to re-run it without a DB
  - omniscient debugging with an editor as the only (and main) UI
    - smart output
    - smart focus
    - text-driven target choice?
    - hyperlinks to jump back / forward
  - only re-run corpus / re-run + explore a bit / re-run + explore more / full fuzzing modes
  - green checkmark meaning current corpus is passing
- fuzzing
  - sage-like "stray off the usual path" exploration model
  - simplex-like / hurst-exponent-like modeling of correlated rare events
  - simulated annealing to guide search

### Misc.

- logo (project + vscode language icon)

### VSCode

- remove hello command
- setup testing?
- decide what declarative features we have to provide
