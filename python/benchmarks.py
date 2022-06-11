# Benchmarking for ACDS
from damage_acds import damage_reduction
from typing import List, Tuple
import timeit

def damage_reduction_early_exit(damage: int, dr_bytes: List[int], non_zero: int):
    """Alternative that tracks number of non-zero indexes for early exit."""
    for (numerator, count) in enumerate(dr_bytes, start=1):
        if non_zero == 0:
            break
        if count > 0:
            for i in range(0, count):
                damage -= damage * numerator // 8
            non_zero -= 1
        elif count < 0:
            for i in range(0, count, -1):
                damage += damage * numerator // 8
            non_zero -= 1
            
    return damage


def damage_reduction_list(damage: int, dr_fractions: List[Tuple[int, int]]):
    """Alternative using list of (numerator, denominator) fractional values."""
    for numerator, denominator in dr_fractions:
        damage -= damage * numerator // denominator

    return damage


def bench_acds_default(suite: List):
    for dr, _ in suite:
        dmg = damage_reduction(1000, dr)


def bench_acds_early_exit(suite: List):
    for dr, non_zero in suite:
        dmg = damage_reduction_early_exit(1000, dr, non_zero)


def bench_list(suite: List):
    for dr in suite:
        dmg = damage_reduction_list(1000, dr)


if __name__ == "__main__":
    print(f"\n----- ACDS Benchmark -----\n")

    empty_acds = [
      #     1/4   1/2   3/4   8/8  nz
        ([0, 0, 0, 0, 0, 0, 0, 0], 0),
        ([0, 0, 0, 0, 0, 0, 0, 0], 0),
        ([0, 0, 0, 0, 0, 0, 0, 0], 0),
        ([0, 0, 0, 0, 0, 0, 0, 0], 0),
        ([0, 0, 0, 0, 0, 0, 0, 0], 0),
        ([0, 0, 0, 0, 0, 0, 0, 0], 0),
        ([0, 0, 0, 0, 0, 0, 0, 0], 0),
        ([0, 0, 0, 0, 0, 0, 0, 0], 0),
    ]

    empty_list = [
        [],
        [],
        [],
        [],
        [],
        [],
        [],
        [],
    ]

    light_acds = [
      #     1/4   1/2   3/4   8/8  nz
        ([0, 0, 0, 0, 0, 0, 0, 0], 0),
        ([0, 0, 0, 0, 0, 0, 0, 0], 0),
        ([0, 0, 0, 0, 0, 0, 0, 0], 0),
        ([0, 0, 0, 0, 0, 0, 0, 0], 0),
        ([0, -1, 0, 0, 0, 0, 0, 0], 1),
        ([0, 0, 0, -1, 0, 0, 0, 0], 1),
        ([0, 0, 0, 0, 0, 1, 0, 0], 1),
        ([0, 0, 0, 0, 0, 0, 0, 1], 1),
    ]

    light_list = [
        [],
        [],
        [],
        [],
        [(-1, 4)],
        [(-1, 2)],
        [(3, 4)],
        [(1, 1)],
    ]

    medium_acds = [
      #     1/4   1/2   3/4   8/8  nz
        ([-1, 0, 0, 0, 0, 0, 0, 0], 1),
        ([0, -1, 0, 0, 0, 0, 0, 0], 1),
        ([0, 0, 1, 0, 0, 0, 0, 0], 1),
        ([0, 0, 0, 1, 0, 0, 0, 0], 1),
        ([0, 0, 0, 0, 1, 1, 0, 0], 2),
        ([0, 0, 0, 0, 0, 1, 1, 0], 2),
        ([0, 0, 1, 1, 0, 0, 0, 0], 2),
        ([0, 1, 1, 0, 0, 0, 0, 0], 2),
    ]

    medium_list = [
        [(-1, 8)],
        [(-1, 4)],
        [(3, 8)],
        [(1,2)],
        [(5, 8), (3, 4)],
        [(3, 4), (7, 8)],
        [(3, 8), (1, 2)],
        [(1, 4), (3, 8)],
    ]

    heavy_acds = [
      #     1/4   1/2   3/4   8/8  nz
        ([1, 1, 1, 1, 1, 0, 0, 0], 5),
        ([0, 1, 1, 1, 1, 1, 0, 0], 5),
        ([0, 0, 1, 1, 1, 1, 1, 0], 5),
        ([0, 0, 0, 1, 1, 1, 1, 1], 5),
        ([-1, 0, 2, 0, 1, 0, 1, 0], 4),
        ([0, -1, 0, 2, 0, 1, 0, 1], 4),
        ([1, 0, -1, 0, 2, 0, 1, 0], 4),
        ([0, 1, 0, -1, 0, 2, 0, 1], 4),
    ]

    heavy_list = [
        [(1, 8), (1, 4), (3, 8), (1, 2), (5, 8)],
        [(1, 4), (3, 8), (1, 2), (5, 8), (3, 4)],
        [(3, 8), (1, 2), (5, 8), (3, 4), (7, 8)],
        [(1, 2), (5, 8), (3, 4), (7, 8), (1, 1)],
        [(-1, 8), (3, 8), (3, 8), (5, 8), (7, 8)],
        [(-1, 4), (1, 2), (1, 2), (3, 4), (1, 1)],
        [(1, 8), (-3, 8), (5, 8), (5, 8), (7, 8)],
        [(1, 4), (-1, 2), (3, 4), (3, 4), (1, 1)],
    ]

    # Default Times
    time_empty_default = timeit.timeit(
        "bench_acds_default(empty_acds)", 
        setup="from __main__ import empty_acds, bench_acds_default"
    )

    time_light_default = timeit.timeit(
        "bench_acds_default(light_acds)", 
        setup="from __main__ import light_acds, bench_acds_default"
    )

    time_medium_default = timeit.timeit(
        "bench_acds_default(medium_acds)", 
        setup="from __main__ import medium_acds, bench_acds_default"
    )

    time_heavy_default = timeit.timeit(
        "bench_acds_default(heavy_acds)", 
        setup="from __main__ import heavy_acds, bench_acds_default"
    )    

    # Early Exit Times
    time_empty_early_exit = timeit.timeit(
        "bench_acds_early_exit(empty_acds)", 
        setup="from __main__ import empty_acds, bench_acds_early_exit"
    )

    time_light_early_exit = timeit.timeit(
        "bench_acds_early_exit(light_acds)", 
        setup="from __main__ import light_acds, bench_acds_early_exit"
    )

    time_medium_early_exit = timeit.timeit(
        "bench_acds_early_exit(medium_acds)", 
        setup="from __main__ import medium_acds, bench_acds_early_exit"
    )

    time_heavy_early_exit = timeit.timeit(
        "bench_acds_early_exit(heavy_acds)", 
        setup="from __main__ import heavy_acds, bench_acds_early_exit"
    )    

    # List Times
    time_empty_list = timeit.timeit(
        "bench_list(empty_list)", 
        setup="from __main__ import empty_list, bench_list"
    )

    time_light_list = timeit.timeit(
        "bench_list(light_list)", 
        setup="from __main__ import light_list, bench_list"
    )

    time_medium_list = timeit.timeit(
        "bench_list(medium_list)", 
        setup="from __main__ import medium_list, bench_list"
    )

    time_heavy_list = timeit.timeit(
        "bench_list(heavy_list)", 
        setup="from __main__ import heavy_list, bench_list"
    )        

    print(f"Empty:")
    print(f"  Default:    {time_empty_default}")
    print(f"  Early Exit: {time_empty_early_exit}")
    print(f"  List:       {time_empty_list}")

    print(f"Light:")
    print(f"  Default:    {time_light_default}")
    print(f"  Early Exit: {time_light_early_exit}")
    print(f"  List:       {time_light_list}")

    print(f"Medium:")
    print(f"  Default:    {time_medium_default}")
    print(f"  Early Exit: {time_medium_early_exit}")
    print(f"  List:       {time_medium_list}")

    print(f"Heavy:")
    print(f"  Default:    {time_heavy_default}")
    print(f"  Early Exit: {time_heavy_early_exit}")
    print(f"  List:       {time_heavy_list}")
