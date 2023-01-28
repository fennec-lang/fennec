# fennec

- simple C/Go/Rust inspired core
- no GC, no runtime (or lightweight one)
- aim for Go-like speed, maybe a bit faster
- always live, always runnable, even with syntax or type errors
- complete and total determinism of execution
- always fuzzing (white-box) in the background, update fuzzing corpus (in the source code form) automatically
- no complex proof-like safety systems, all-in on fuzzing to found any problems
- built-in sanitizers (ASAN etc.); fat (unspecified size) pointers/references to assist in checking
- no implicit environment of any kind, capability-secure APIs by default
- generic from day 1 (but very simple generics)
- explicit object lifetime (create/destroy/copy/move), tied to scopes
- no invisible code (destructors); `defer` instead (destructors with effects are possible)
- default to strongly const (but no WxA; so have to pessimistically re-read const things)
- `require` + `assert` in prelude, distinguish preconditions and internal invariant violations
- think about `debug` assertions which check complex invariants but are off by default
- no null, no pointer arithmetic & casts, throw on overflow and OOB
- expose throw as a control flow construct
- model complex systems as single thread agents with epoll-like interface; maybe some structured concurrency later
- start with core fennec for language, and have minimum things built-in (rely on prelude instead)

## misc. questions

- built-in flexible arrays and DSTs? with linked size field nearby
  - probably little need, since we can define a vector (or even `smallvec`!)
- have a default global allocator or not? `any` etc. require it, kind of
