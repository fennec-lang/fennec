# tables

- all state is in tables
- triggers on everything
- views (& materialized views), indexes (& materialized ones)
- row vs column layout, some control over data locality
- no idea how a table-based low-level imperative language would look
- resources in tables, too!

- all state is explicit and central to a package
  - encapsulation inside a package is bad
  - package == unit of no encapsulation and flat state
- enforce that all state is in one file?
  - we can have all "one-for-package" metadata inside the same file
  - we can enforce that its name is the same as the package name
- do imports from different modules get the same or different state?

- does this all somehow mesh with W^A thing from rust?
  - query vs command separation
- and what is the difference with regular collections?
  - very limited nesting
  - guaranteed data invariants
    - and consistent projections
  - whole-picture visibility
  - transactions?
  - primary key = pointer?

- all state is external to functions
  - how do functions find things they were working on?
- does this relate to ECS?
  - entity = row
  - column = component
  - ecs == implicit table-like schema?
  - send data between systems via channels
  - change detection vs triggers
- programming = algorithms & data structures, aren't we limiting data structures?
  - queues
  - implicit (intrusive?) data structures

- built-in ecs-like thing:
  - want direct support for states, relationships, some encapsulation
  - transitive, reflexive, acyclic relationship properties
