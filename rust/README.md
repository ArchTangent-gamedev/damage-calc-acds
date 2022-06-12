# ACDS Damage Calculations (Rust)
System for storing and calculating damage modification using the *Additive Common Divisor System* (ACDS), in Rust.

For more details, see the parent `README.md`.

## Testing

Unit testing available via `cargo test`.

## Benchmarks

The benchmark compares the default ACDS implementation against two alternatives:
1. `List`: uses a list of `(numerator, denominator)` pairs of unbounded size (not a fixed 8 bytes like the default implementation).
2. `Early Exit`: keeps track of the number of `non_zero` indexes, and exits once all such indexes have been checked.

Four scenarios are tested:
1. `Empty`: no damage modifiers.
2. `Light`:  one damage modifier per entry.
3. `Medium`:  two damage modifiers per entry.
4. `Heavy`: five damage modifiers per entry.

Benchmarks are stored in `/benches`, and performed via `cargo bench`.

### Results

Empty:
```
Default:      4.82 ns
Early Exit:   1.21 ns
List:         1.11 ns
```

Light:
```
Default:     46.74 ns
Early Exit:  27.55 ns
List:        23.08 ns
```

Medium:
```
Default:     51.95 ns
Early Exit:  40.22 ns
List:        41.25 ns
```

Heavy:
```
Default:     66.83 ns
Early Exit:  67.32 ns
List:       141.66 ns
```

Observations:
- The `list` method is fastest for `empty` and `light` scenarios.
- The `early_exit` method is fastest for `medium` scenarios.
- The `default` method is fastest for `heavy` scenarios, and sufficiently fast in all.  It requires no updating or tracking of extra data (`non_zero` for `early_exit`), and uses constant space (unlike `list`, which is unbounded in size).
