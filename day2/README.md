# Day 2

All you need is a parser (generator). I use [pest.rs](https://pest.rs/), which is a PEG parser generator for Rust.

The grammar is as follows (see also [game.pest](game.pest)):

```pest
file         = { (game ~ NEWLINE?)+ }
game         = { "Game" ~ game_id ~ ":" ~ game_content }
game_id      = { ASCII_DIGIT+ }
game_content = { game_set* }

game_set      =  { cube_data+ ~ optional_semi }
optional_semi = _{ ";"? }

cube_data      =  { cube_count ~ cube_color ~ optional_comma }
cube_count     =  { ASCII_DIGIT+ }
cube_color     =  { ASCII_ALPHA+ }
optional_comma = _{ ","? }

ASCII_DIGIT = _{ '0'..'9' }
ASCII_ALPHA = _{ 'a'..'z' | 'A'..'Z' }

WHITESPACE = _{ " " }
```

See rust code in [src/main.rs](src/main.rs).
