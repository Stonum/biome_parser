# biome_parser (fork)

A fork of [`biome_parser`](https://crates.io/crates/biome_parser) **0.5.8** — Biome's
shared parser infrastructure — with a larger limit on the number of token kinds and a fix
for token kinds above 127.

## Why

`TokenSet` is a fixed-size bit set: upstream it holds `[u128; 2]`, i.e. at most **256**
token kinds per language. A full SQL grammar (Postgres DDL and PL/pgSQL keywords) needs
more than that.

While widening it, an off-by-one in `TokenSet`'s mask was found: for kinds above 127 it
computed the bit as `num - 127` instead of `num - 128`, so such kinds landed on the wrong
bit and sets could report false matches.

## What changed

Only `src/token_set.rs`:

- the bit set is widened to `[u128; 4]` (`TOKEN_SET_WORDS = 4`), allowing **512** token
  kinds;
- the mask is computed per 128-bit word, which also fixes the off-by-one above.

The API and the version number (`0.5.8`) are identical to the crates.io release, so the
fork is a drop-in replacement.

See the commit history: the first commit is the unmodified crates.io source, the second
one is the change.

## Usage

Use a `[patch.crates-io]` entry so every crate that depends on `biome_parser` picks up the
fork:

```toml
[patch.crates-io]
biome_parser = { git = "https://github.com/Stonum/biome_parser.git" }
```

It is used together with the [biome_rowan fork](https://github.com/Stonum/biome_rowan),
which lifts the matching limit on syntax kinds.

## License

Same as upstream Biome: dual-licensed under [MIT](LICENSE-MIT) or
[Apache-2.0](LICENSE-APACHE), at your option. Original work © Biome Developers and
Contributors.

The crates.io package of `biome_parser` 0.5.8 ships without license files; the ones in
this repository are taken from the Biome project (identical to those shipped with
`biome_rowan` 0.5.8).
