# TODO

- re-bootstrap the project: no dependencies + agent-first
  - start with finishing chaos_theory
  - then enable rust bootstrap
    - reference implementations fully decoupled from i/o and real world
      - LSP etc. will be attached via a shell (module-like design)
  - focus on MIR (core typed imperative semantics)
    - MIR has does not require a parser/etc, but can be executed (via reference interpreter) and type-checked
      - write this code first
      - this *is* the language spec
      - start with pointer/place/value, at the very bottom; express in terms of traits & contracts
        - build our own MLIR-like stack on top, without leaving the language to describe itself
        - compile by doing a partial interpretation of reference interpreter execution pointer/value specs?
    - HIR is a sugar layer on top
      - contract = lowering to MIR
      - effects via capability objects and compiler auto-fill of them
        - including generators & async/await (tree-structured of course)
          - code should just specify control flow from a user PoV; how this maps to real stacks does not matter at all
    - LIR is lowering to assembly/WASM below MIR
      - contract = preserving reference MIR semantics
  - enter bootstrap ASAP (right after reference MIR interpreter?) to dogfood
  - python + rust into one language


- goal: full-stack pure imperative language
  - write decoupled bricks (modules) that don't assume anything
  - performant and rich enough to fully self-bootstrap
  - performant and rich enough to build a real OS
    - start from safe PCI/DMA controller access and build up
    - streams = virtio queues, but typed? also = pipes
- vision: small imperative core that is pure, collection of reusable pure modules that can be arbitrarily composed
  - need to be able to do boost::graph but in a good way
  
- start with a bit: a place and a value
  - model place invalidation and values moving between places
  - get memory safety (R^W regions, free aliasing in a region)

- sans-io is fundamental
  - pure computation that is reliable & controllable, plus IO (= communication) that is
    always unreliable and un-controllable
  - make this first-class; force to think in terms of submit queues, command buffers and the like
  - agent paradigm done right

- if we have a deep integration of design by contract and comptime, we have static type checking without any type system
  - type-checking is just a comptime evaluation, and design by contract gives us blame assignment at the language level
    - also integrate clojure.spec in there at the core level
