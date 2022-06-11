//! Additive Common Divisor System (ACDS) for damage calcs.
//! 
//! There are two ACDS contexts:
//! - Damage Bonus (DB): used for dealing damage
//! - Damage Reduction (DR): used for reducing damaage

/// Returns damage after damage bonus. (+) raises damage, (-) lowers it.
pub fn damage_bonus(damage: i32, db_btyes: [i8; 8]) -> i32 {
    let mut output = damage;
    let mut numerator = 0;

    for &count in db_btyes.iter() {
        numerator += 1;

        if count == 0 {
            continue
        }
        if count > 0 {
            for _bonus in 0..count {
                output += output * numerator / 8
            }
        } else if count < 0 {
            for _bonus in count..0 {
                output -= output * numerator / 8
            }
        }
    }
    return output
}

/// Returns damage after damage reduction. (+) lowers damage, (-) raises it.
pub fn damage_reduction(damage: i32, dr_bytes: [i8; 8]) -> i32 {
    let mut output = damage;
    let mut numerator = 0;

    for &count in dr_bytes.iter() {
        numerator += 1;

        if count == 0 {
            continue
        }
        if count > 0 {
            for _reduction in 0..count {
                output -= output * numerator / 8
            }
        } else if count < 0 {
            for _reduction in count..0 {
                output += output * numerator / 8
            }
        }
    }
    return output
}

/// Converts nominal damage bonus value into byte list form.
pub fn db_bytes_from_nominal(nominal: i8) -> [i8; 8] {
    let mut db_bytes: [i8; 8] = [0; 8];

    let double = nominal / 8;
    let fraction = (nominal.abs() % 8) as usize;

    db_bytes[7] = double;

    if fraction != 0 {
        db_bytes[fraction - 1] = if nominal > 0 { 1 } else { -1 };
    }

    db_bytes
}

/// Converts nominal damage reduction value into byte list form.
pub fn dr_bytes_from_nominal(nominal: i8) -> [i8; 8] {
    let mut dr_bytes: [i8; 8] = [0; 8];

    let immunity = nominal / 8;
    let fraction = (nominal.abs() % 8) as usize;

    dr_bytes[7] = immunity;

    if fraction != 0 {
        dr_bytes[fraction - 1] = if nominal > 0 { 1 } else { -1 };
    }

    dr_bytes
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn acds_damage_bonus() {
        let nominal_list: Vec<i8> = (-10..=10).collect();
        let db_byte_list: Vec<[i8;8]> = vec![
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
        ];
        let damages = [
            0, 0, 0, 125, 250, 375, 500, 625, 750, 875, 1000, 
            1125, 1250, 1375, 1500, 1625, 1750, 1875, 2000, 2250, 2500
        ];        

        for (nominal, expected_bytes) in nominal_list.iter().zip(&db_byte_list) {
            let actual = db_bytes_from_nominal(*nominal);
            assert_eq!(actual, *expected_bytes)
        }

        for (db_bytes, expected_damage) in db_byte_list.iter().zip(damages) {
            let actual = damage_bonus(1000, *db_bytes);
            assert_eq!(actual, expected_damage)  
        }
    }

    #[test]
    fn acds_damage_reduction() {
        let nominal_list: Vec<i8> = (-10..=10).collect();
        let dr_byte_list: Vec<[i8;8]> = vec![
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
        ];
        let damages = [
            2500, 2250, 2000, 1875, 1750, 1625, 1500, 1375, 1250, 1125, 1000,
            875, 750, 625, 500, 375, 250, 125, 0, 0, 0
        ];

        for (nominal, expected_bytes) in nominal_list.iter().zip(&dr_byte_list) {
            let actual = dr_bytes_from_nominal(*nominal);
            assert_eq!(actual, *expected_bytes)
        }

        for (dr_bytes, expected_damage) in dr_byte_list.iter().zip(damages) {
            let actual = damage_reduction(1000, *dr_bytes);
            assert_eq!(actual, expected_damage)  
        }
    }

    #[test]
    fn acds_compound_damage_reduction() {
        let dr_byte_list: Vec<[i8;8]> = vec![
            [2, 0, 0, 0, 0, 0, 0, 0],   // 766 (23.4% DR)
            [0, 0, 0, 0, 0, -3, 1, 0],  // 670 (33.0% DR)
            [1, 1, 0, 0, 0, 0, 0, 0],   // 657 (34.3% DR)
            [0, 2, 0, 0, 0, 0, 0, 0],   // 563 (43.7% DR)
            [0, 0, 2, 0, 0, 0, 0, 0],   // 391 (60.9% DR)
            [0, 1, 0, 1, 0, 0, 0, 0],   // 375 (62.5% DR)
            [0, 0, 0, 0, -2, 0, 1, 0],  // 330 (67.0% DR)
        ];
        let damages = [766, 670, 657, 563, 391, 375, 330];

        for (dr_bytes, expected_damage) in dr_byte_list.iter().zip(damages) {
            let actual = damage_reduction(1000, *dr_bytes);
            assert_eq!(actual, expected_damage)  
        }  
    }
}
