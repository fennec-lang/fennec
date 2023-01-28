# index-based access

- index is not self-sufficient, requires the object for access
  - but can check safety at the time of access
- usages:
  - MVS (swift, val)
  - boost graph library
  - rust slotmap and the like

- can we design a system where we hold references (indices) declaratively?
  - compiler/runtime can help with safety, if it has high-level understanding of it
  - like datadraw references, but for everything
  - my idea of "lightweight lifetimes" via explicit annotation of relationships is kinda about the same

- so, goals:
  - high-level, declarative specification of
    - what owns what
    - what relates to what
    - what is subpart of what
  - based on the spec, appropriate safe access and modification methods are generated
    - you have no other way to access the data
  - so, we have safe-by-construction access, because we can't use unsafe primitives at all
  - what kind of safety we want:
    - null safety
    - iterator invalidation (can't use an invalid iterator)
    - memory safety (no use after free, no double free)
- what is left over is to design the whole thing :-)
