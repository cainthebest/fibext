# fibext

`fibext` is a versatile Fibonacci sequence generator for Rust, offering support for various features and customization options.

## Features

- Support for different types of unsigned integers.
- Option to enable checked overflow for arithmetic operations.
- Support for large numbers using the `num_bigint` crate (enabled through the `large-numbers` feature).
- Iterator implementation for generating Fibonacci sequences (enabled through the `iterator` feature).

## Usage

Add `fibext` as a dependency in your `Cargo.toml` file:

```toml
[dependencies]
fibext = "0.2.0"
```

Import the `fibext` crate into your Rust code:

```rust
use fibext::*;
```

Create a new Fibonacci sequence:

```rust
let fib: Fibonacci<u32> = Fibonacci::new();
```

Iterate over the Fibonacci sequence:

```rust
for number in fib.take(10) {
println!("{}", number);
}
```

This will print the first 10 Fibonacci numbers.

## Supported Types

The `fibext` library supports the following types of unsigned integers:

- `u8`
- `u16`
- `u32`
- `u64`
- `u128`

When the `large-numbers` feature is enabled, the library also supports `BigUint` from the `num_bigint` crate.

## Optional Features

The `fibext` library provides several optional features that can be enabled or disabled based on your needs. These features are controlled through the `features` section in your `Cargo.toml` file.

- `std` (enabled by default): Enables the use of `std` types and features. When disabled, the library uses the core version of `Wrapping` and does not rely on `std`.
- `checked-overflow` (enabled by default): Enables checked overflow for arithmetic operations. When enabled, the library returns an `ArithmeticError` if an overflow occurs.
- `iterator` (enabled by default): Enables the iterator implementation for generating Fibonacci sequences.
- `large-numbers` (optional): Enables support for large numbers using the `num_bigint` crate. To enable this feature, add the `large-numbers` feature under the `features` section in your `Cargo.toml` file.

```toml
[dependencies]
fibext = { version = "0.2.0", features = ["large-numbers"] }
```

## Benchmarks

The `fibext` library includes a benchmark for Fibonacci sequence generation. To run the benchmark, use the following command:

```shell
cargo bench --bench fibonacci
```

## License

This project is licensed under the GNU General Public License v3.0. See the [LICENSE](https://github.com/cainthebest/fibext/blob/master/LICENSE) file for more details.
