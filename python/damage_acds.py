# Additive Common Divisor System (ACDS) for damage bonus and reduction.
#
# There are two ACDS contexts:
# - Damage Bonus (DB): used for dealing damage
# - Damage Reduction (DR): used for reducing damaage
from typing import List

def damage_bonus(damage: int, dr_bytes: List[int]):
    """Returns damage after damage bonus. (+) raises damage, (-) lowers it."""
    for (numerator, count) in enumerate(dr_bytes, start=1):
        if count == 0:
            continue
        if count < 0:
            for i in range(0, count, -1):
                damage -= damage * numerator // 8
        else:
            for i in range(0, count):
                damage += damage * numerator // 8

    return damage


def damage_reduction(damage: int, dr_bytes: List[int]):
    """Returns damage after damage reduction. (+) lowers damage, (-) raises it."""
    for (numerator, count) in enumerate(dr_bytes, start=1):
        if count == 0:
            continue        
        if count < 0:
            for i in range(0, count, -1):
                damage += damage * numerator // 8
        else:
            for i in range(0, count):
                damage -= damage * numerator // 8

    return damage


def db_bytes_from_nominal(nominal: int) -> List[int]:
    """Converts nominal Damage Bonus value into byte list form.
    
    Examples:
    -  1: [1, 0, 0, 0, 0, 0, 0, 0] =  12.5% DB
    -  2: [0, 1, 0, 0, 0, 0, 0, 0] =  25.0% DB
    -  7: [0, 0, 0, 0, 0, 0, 1, 0] =  87.5% DB
    -  8: [0, 0, 0, 0, 0, 0, 0, 1] = 100.0% DB 
    - 10: [0, 1, 0, 0, 0, 0, 0, 1] = 150.0% DB
    - 19: [0, 0, 1, 0, 0, 0, 0, 2] = 300.0% DB

    Note: abs() is used to avoid issues w/ Python and negative number division.
    """
    db_bytes = [0, 0, 0, 0, 0, 0, 0, 0]

    double = abs(nominal) // 8
    fraction = abs(nominal) % 8
    if double:
        db_bytes[7] = double if nominal > 0 else -double
    if fraction:
        db_bytes[fraction - 1] = 1  if nominal > 0 else -1

    return db_bytes


def dr_bytes_from_nominal(nominal: int) -> List[int]:
    """Converts nominal Damage Reduction value into byte list form.
    
    Examples:
    -  1: [1, 0, 0, 0, 0, 0, 0, 0] =  12.5% DR
    -  2: [0, 1, 0, 0, 0, 0, 0, 0] =  25.0% DR
    -  7: [0, 0, 0, 0, 0, 0, 1, 0] =  87.5% DR
    -  8: [0, 0, 0, 0, 0, 0, 0, 1] = 100.0% DR 
    - 10: [0, 1, 0, 0, 0, 0, 0, 1] = 150.0% DR
    - 19: [0, 0, 1, 0, 0, 0, 0, 2] = 300.0% DR

    Note: abs() is used to avoid issues w/ Python and negative number division.
    """
    dr_bytes = [0, 0, 0, 0, 0, 0, 0, 0]

    immunity = abs(nominal) // 8
    fraction = abs(nominal) % 8
    if immunity:
        dr_bytes[7] = immunity if nominal > 0 else -immunity
    if fraction:
        dr_bytes[fraction - 1] = 1  if nominal > 0 else -1

    return dr_bytes

#   ########  ########   ######   ########
#      ##     ##        ##           ##
#      ##     ######     ######      ##
#      ##     ##              ##     ##
#      ##     ########  #######      ## 

def test_acds_damage_bonus():
    """Ensure proper creation and damage bonus of DB bytes (damage = 1000)."""    
    nominal_list = [n for n in range(10, -11, -1)]
    db_byte_list = [
        [0, 1, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 1],
        [0, 0, 0, 0, 0, 0, 0, 1],
        [0, 0, 0, 0, 0, 0, 1, 0],
        [0, 0, 0, 0, 0, 1, 0, 0],
        [0, 0, 0, 0, 1, 0, 0, 0],
        [0, 0, 0, 1, 0, 0, 0, 0],
        [0, 0, 1, 0, 0, 0, 0, 0],
        [0, 1, 0, 0, 0, 0, 0, 0],
        [1, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [-1, 0, 0, 0, 0, 0, 0, 0],
        [0, -1, 0, 0, 0, 0, 0, 0],
        [0, 0, -1, 0, 0, 0, 0, 0],
        [0, 0, 0, -1, 0, 0, 0, 0],
        [0, 0, 0, 0, -1, 0, 0, 0],
        [0, 0, 0, 0, 0, -1, 0, 0],
        [0, 0, 0, 0, 0, 0, -1, 0],
        [0, 0, 0, 0, 0, 0, 0, -1],
        [-1, 0, 0, 0, 0, 0, 0, -1],
        [0, -1, 0, 0, 0, 0, 0, -1],
    ]
    damages = [
        2500, 2250, 2000, 1875, 1750, 1625, 1500, 1375, 1250, 1125, 1000,
        875, 750, 625, 500, 375, 250, 125, 0, 0, 0
    ]
    for nominal, expected_bytes in zip(nominal_list, db_byte_list):
        actual = db_bytes_from_nominal(nominal)
        assert actual == expected_bytes

    for db_bytes, expected_damage in zip(db_byte_list, damages):
        actual = damage_bonus(1000, db_bytes)
        assert actual == expected_damage   


def test_acds_damage_reduction():
    """Ensure proper creation and damage reduction of DR bytes (damage = 1000)."""    
    nominal_list = [n for n in range(-10, 11)]
    dr_byte_list = [
        [0, -1, 0, 0, 0, 0, 0, -1],
        [-1, 0, 0, 0, 0, 0, 0, -1],
        [0, 0, 0, 0, 0, 0, 0, -1],
        [0, 0, 0, 0, 0, 0, -1, 0],
        [0, 0, 0, 0, 0, -1, 0, 0],
        [0, 0, 0, 0, -1, 0, 0, 0],
        [0, 0, 0, -1, 0, 0, 0, 0],
        [0, 0, -1, 0, 0, 0, 0, 0],
        [0, -1, 0, 0, 0, 0, 0, 0],
        [-1, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [1, 0, 0, 0, 0, 0, 0, 0],
        [0, 1, 0, 0, 0, 0, 0, 0],
        [0, 0, 1, 0, 0, 0, 0, 0],
        [0, 0, 0, 1, 0, 0, 0, 0],
        [0, 0, 0, 0, 1, 0, 0, 0],
        [0, 0, 0, 0, 0, 1, 0, 0],
        [0, 0, 0, 0, 0, 0, 1, 0],
        [0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 1],
        [0, 1, 0, 0, 0, 0, 0, 1],
    ]
    damages = [
        2500, 2250, 2000, 1875, 1750, 1625, 1500, 1375, 1250, 1125, 1000,
        875, 750, 625, 500, 375, 250, 125, 0, 0, 0
    ]
    for nominal, expected_bytes in zip(nominal_list, dr_byte_list):
        actual = dr_bytes_from_nominal(nominal)
        assert actual == expected_bytes

    for dr_bytes, expected_damage in zip(dr_byte_list, damages):
        actual = damage_reduction(1000, dr_bytes)
        assert actual == expected_damage        


def test_acds_compouned_damage_reduction():
    """Test DR for compound values - more than one non-zero index (damage = 1000)."""    
    dr_byte_list = [
        [2, 0, 0, 0, 0, 0, 0, 0],   # 766 (23.4% DR)
        [0, 0, 0, 0, 0, -3, 1, 0],  # 670 (33.0% DR)
        [1, 1, 0, 0, 0, 0, 0, 0],   # 657 (34.3% DR)
        [0, 2, 0, 0, 0, 0, 0, 0],   # 563 (43.7% DR)
        [0, 0, 2, 0, 0, 0, 0, 0],   # 391 (60.9% DR)
        [0, 1, 0, 1, 0, 0, 0, 0],   # 375 (62.5% DR)
        [0, 0, 0, 0, -2, 0, 1, 0],  # 330 (67.0% DR)
    ]
    damages = [766, 670, 657, 563, 391, 375, 330]

    for dr_bytes, expected_damage in zip(dr_byte_list, damages):
        actual = damage_reduction(1000, dr_bytes)
        assert actual == expected_damage        
