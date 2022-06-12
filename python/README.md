# ACDS Damage Calculations (Python)
System for storing and calculating damage modification using the *Additive Common Divisor System* (ACDS), in Python.

For more details, see the parent `README.md`.

## Testing

Ensure `pytest` is installed, then enter the following:
- `pytest damage_acds.py`

## Benchmarks

The benchmark compares the default ACDS implementation against two alternatives:
1. `List`: uses a list of `(numerator, denominator)` pairs of unbounded size (not a fixed 8 bytes like the default implementation).
2. `Early Exit`: keeps track of the number of `non_zero` indexes, and exits once all such indexes have been checked.

Four scenarios are tested:
1. `Empty`: no damage modifiers.
2. `Light`:  half of entries are empty, half have one damage modifier.
3. `Medium`:  half of entries have one damage modifier, half have two.
4. `Heavy`: all entries have 5 damage modifiers.

See `benchmarks.py`.

### Results

Empty:
```
Default:     4.07 seconds
Early Exit:  2.11 seconds
List:        1.37 seconds
```
Light:
```
Default:     4.88 seconds
Early Exit:  4.37 seconds
List:        2.01 seconds
```
Medium:
```
Default:     7.56 seconds
Early Exit:  7.59 seconds
List:        1.79 seconds
```  
Heavy:
```
Default:    12.99 seconds
Early Exit: 14.29 seconds
List:        3.54 seconds
```

The `list` method is the fastest in all cases, coming at the cost of space (unbounded).

`Early exit` provides a slight speed edge over the default test case, but is about even (or slightly worse) in other cases.  It also requires an extra value (`non_zero`) to be tracked at all times, adding complexity.
