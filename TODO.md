# TODO

- strong testing infrastructure to build upon
- replace example parser tests with something meaningful
- hook parser to REPL
- create an interpreter (or should we start with real compiler immediately?)

## Misc.

- think about how to make it easy to convert production trace to a test (automatically)
- polish as you go (and fix bugs immediately), always maintain shippable software
- develop on 64-bit only, run on 32-bit too

## Language

- tower-of-languages (from most restrictive to less)
- unsafe blocks, otherwise safe? overflows, bounds checks, data race (?), non-null, 
- wasm first class target
- FFI via WASM embeddings

## Sandbox

- multiple isolated (wasm-style parts of the program)
- pointers are local by default
- no global address space
- shared-nothing instantiatable components (wasm-components-like)
- unforgeable references for capabilities
- command vs reactor modules? or just make everything a reactor?
