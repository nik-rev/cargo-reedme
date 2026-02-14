TOML lexer and parser

Characteristics:
- Error recovery
- Lazy validation
- `forbid(unsafe)` by default, requiring the `unsafe` feature otherwise
- `no_std` support, including putting users in charge of allocation choices (including not
  allocating)

Full parsing is broken into three phases:
1. [lexer](https://docs.rs/toml_parser/1.0.8+spec-1.1.0/toml_parser/lexer/)
2. [parser](https://docs.rs/toml_parser/1.0.8+spec-1.1.0/toml_parser/parser/) (push parser)
3. Organizing the physical layout into the logical layout,
   including [decoder](https://docs.rs/toml_parser/1.0.8+spec-1.1.0/toml_parser/decoder/)