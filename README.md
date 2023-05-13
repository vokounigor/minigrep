# minigrep

Implementation of a grep-like program in Rust following the [Rust Book example](https://doc.rust-lang.org/book/ch12-00-an-io-project.html).

### Running
To run the program, run:
```sh
cargo run -- [file_path] [query] <[-c]>
```

`-c` is optional, it runs a case-insensitive search. Same can be achieved running:

```sh
IGNORE_CASE=1 cargo run -- [file_path] [query]
```

### Test
To run the tests, run
```sh
cargo test
```
