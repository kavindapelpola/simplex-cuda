# Simplex and other lp solvers in Rust

## Build

```
cargo build --release
```

## Examples

Run the examples
```
cargo run --example simplex_cuda
```

## Benchmarks

Run all the benchmarks
```
cargo bench
```

Run a group of benchmarks
```
cargo bench --bench <group>
eg.
cargo bench --bench flat_matrix
```

Run a specific bench in a group
```
cargo bench --bench <group> -- "<bench>"
eg.
cargo bench --bench flat_matrix -- "get"
```
