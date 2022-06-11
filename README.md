# ACDS Damage Calculations
System for storing and calculating damage modification using ACDS, in Python and Rust.

## About ACDS

ACDS stands for *Additive Common Divisor System*.  It allows for damage modification data to be stored in an array (or list, in Python) of eight bytes (or integers in Python).  This has some advantages in particular use cases.

### Uses

Use cases include:
- damage bonus for attacks
- damage reduction for defense

### Terminology

`DB`: damage bonus. Used in the context of attacking.

`DR`: damage reduction. Used in the context of defending.

`DB/DR bytes`: list of 8 integers (`i8`s in Rust) representing a DB/DR value.

`DB/DR nominal`: integer representation of a DB/DR bytes value (e.g. 3 = [0,0,1,0,0,0,0,0]).

### Structure

A damage bonus/reduction bytes structure is an array of 8 bytes:

```rust
//                      1/8      1/2      8/8  
let dr_bytes: [i8; 8] = [0, 1, 0, 0, 0, 0, 0];
```

Each increasing index of `dr_bytes` represents two things:
- the index is the `numerator` of the *common divisor*, `8`
- the `value` at the index is the number of times `numerator * value` is *added* to the nominal `damage`.

Hence, the name *Additive Common Divisor System*.

### Pros & Cons

Pros:
- Double-edged: easy to model damage resistance *and* bonuses
- Removable immunity: allows for *damage immunity* to be applied and later removed by modifiying the 8th bit (at index `[7]`)
- Combinable: multiple byte arrays can be combined, allowing for modifications from armor, status effects, item mods, and more
- Levels of DR are more meaningful:  with only 8 base levels the minimum damage modification is `1/8 (12.5%`)
- Reversible: combining byte arrays is reversible with no loss of data (unlike `float`s)
- Fixed size:  multiple damage modifiers can be represented in the same structure
- Compact: only `8` bytes, the size of a 64-bit `float` or `int`
- Flexible: can use multiple non-zero indexes to create DB/DR values other than multiples of `1/8`. For example, `[0, 0, 0, 0, 0, -3, 1, 0]` is a DB/DR of `33.0% (~1/3)`.

Cons:
- Slower: takes longer to calculate damage compared to a single list of floating point or fractional damage modifiers (about *2.5-5.0x* slower - see Python benchmarks)

## Table

ACDS damage modifiers based on *nominal* DR.

*Note: nominal values can exceed those listed below*.

|  DR  |   DR %   | Notes |  DR  |   DR %   | Notes |
|------|----------|-------|------|----------|-------|
|   1  |   12.5   |       |  -1  |  -12.5   |       |
|   2  |   25.0   |       |   2  |   25.0   |       |
|   3  |   37.5   |       |   3  |   37.5   |       |
|   4  |   50.0   |       |   4  |   50.0   |       |
|   5  |   62.5   |       |   5  |   62.5   |       |
|   6  |   75.0   |       |   6  |   75.0   |       |
|   7  |   87.5   |       |   7  |   87.5   |       |
|   8  |  100.0   |  Imm  |   8  |  100.0   |  2x   |
|   9  |  125.0   |  Imm  |   9  |  125.0   |       |
|  10  |  150.0   |  Imm  |  10  |  150.0   |       |
|  11  |  175.0   |  Imm  |  11  |  175.0   |       |
|  12  |  200.0   |  Imm  |  12  |  200.0   |  3x   |
|  13  |  225.0   |  Imm  |  13  |  225.0   |       |
|  14  |  250.0   |  Imm  |  14  |  250.0   |       |
|  15  |  275.0   |  Imm  |  15  |  275.0   |       |
|  16  |  300.0   |  Imm  |  16  |  300.0   |  4x   |
|  17  |  325.0   |  Imm  |  17  |  325.0   |       |
|  18  |  350.0   |  Imm  |  18  |  350.0   |       |
|  19  |  400.0   |  Imm  |  19  |  400.0   |  5x   |
|  20  |  450.0   |  Imm  |  20  |  450.0   |       |
|  21  |  500.0   |  Imm  |  21  |  500.0   |       |

Key:
- `Imm`: a defender would be immune to incoming damage
- `2x, 3x, 4x, 5x`: an attacker would do, or a defender would take, `2-5x` incoming damage.

## Examples

Below examples are used in the context of damage *reduction* (DR).

Example 1: `damage = 100`, `DR_nominal = 10`
- `DR% = 150%` which is `> 100%`
- no damage (target is immune)

Example 2: `damage = 100`, `DR_nominal = 4`
- `DR% = 50%`
- `damage` reduced to `50`

Example 3: `damage = 100`, `DR_nominal = -12`
- `DR% = -200`
- `damage` increased to `300` (triple damage)
