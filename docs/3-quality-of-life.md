# quality of life

- async
- str vs String vs PathBuf vs whatever in signatures
  - `&str` is magical for some reason, `&str: str` is not the same relationship as `&int: int`; `str` is a DST for some reason
- From/Into
  - this allows overloading (nicer names?), but isn't this an abuse of traits?
- Into/AsRef/Burrow and related noisy things in signatures
