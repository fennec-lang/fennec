# TODO

- strong testing infrastructure to build upon
- replace example parser tests with something meaningful
- hook parser to REPL
- create an interpreter (or should we start with real compiler immediately?)
  - MVP should be interpreter, used as a reference in the future

## Misc.

- think about how to make it easy to convert production trace to a test (automatically)
- polish as you go (and fix bugs immediately), always maintain shippable software
- develop on 64-bit only, run on 32-bit too
- generate test data to the source files (re-run as unit tests, no dependency on DB)

## Language

- start with data description language
- tower-of-languages (from most restrictive to less)
- unsafe blocks, otherwise safe? overflows, bounds checks, data race (?), non-null, 
- wasm first class target
  - if wasm is a first class target, why don't we run in a browser from the start?
    - vscode is already available in the browser
- FFI via WASM embeddings
- no edit-compile-run cycle:
  - everything happens automatically in <100ms on every save
  - find new traces, minify existing, re-write the text traces in test files
    all the time, in the background
- owner annotation? or standard "owner" type that is pool of size 1
  - then we don't need owner annotations as references are non-owning
  - standard pool as well

## Sandbox

- multiple isolated (wasm-style parts of the program)
- pointers are local by default
- no global address space
- shared-nothing instantiatable components (wasm-components-like)
- unforgeable references for capabilities
- command vs reactor modules? or just make everything a reactor?
