# DicGen

Generate all possible combinations for given symbols in given interval.

## Install with Cargo

```bash
cargo install dicgen
```

## Usage

An example generating mobile phone numbers (9 digits starting with 6, 7 or 8):

```bash
dicgen --alphabet 0123456789 --init 600000000 --end 899999999 --file phone_numbers.txt
```

Generated file `phone_numbers.txt` will contain:

```text
600000000
600000001
[...]
899999998
899999999
```

## Usage as Rust dependency

Add to your `Cargo.toml` and see examples and reference in [documentation](https://docs.rs/dicgen/).

## License

The Unlicense. See [LICENSE](./LICENSE) for details or visit [unlicense.org web](http://unlicense.org/).
