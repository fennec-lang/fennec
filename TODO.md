# TODO

## Vision

"A language empowering someone to build reliable and efficient software."

- small language with nimble feel
- lightweight typing
- built by induction to always be auto-testable
- total determinism and purity
- instant feedback
- fast interpreter for development, compile to WASM or C with no dependencies for production

## Ideas

- liberal use of assertions
- uncovered code as a warning
- explicit arbitrary choice
  - constants in code and in test scenarios
  - choice of actions in test scenarios
- statement to signal progress to a fuzzer to help guide the search
- ability to guide a fuzzer through a condition by providing an example
- magical var-size primitives to make enumeration possible
- aspect-like properties
  - fully automatic running of random programs; no code necessary to drive
  - properties that bind to runs that satisfy certain conditions and assert certain things
    - sometimes we can cheat an use property condition as a generation instruction
    - this unifies simple properties for functions and complex properties for stateful
      interactions (that can be interleaved with other stateful interactions)
- always test automatically
  - no crashes
  - no leaks
  - no hangs

## Metaprogramming

- since we can run arbitrary code at compile time, this is easy

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
    - execute -> click to freeze the results to get a test -> generalize to property-based test
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

### Miscellaneous

- logo (project + github social previews + opengraph + vscode language icon + vscode extension icon)

### VSCode

- remove hello command
- setup testing with CI
- decide what declarative features we have to provide
