# pokeapi-rs

Sends requests to [PokeAPI](https://pokeapi.co/docs/v2).

Made to illustrate some practical use cases in Rust, like:

- Reading user input from the command-line with [clap](https://docs.rs/clap/latest/clap/)
- Sending HTTP requests with [reqwest](https://docs.rs/reqwest/latest/reqwest/)
- Error handling with [anyhow](https://docs.rs/anyhow/latest/anyhow/) and [thiserror](https://docs.rs/thiserror/latest/thiserror/)
- Unit testing and mocking external APIs with with [wiremock](https://docs.rs/wiremock/latest/wiremock/)

## Usage

```rust
pokeapi-rs get pikachu
```
