use std::str::FromStr;
use crate::{
    Rounding,
    constants::{
        INVALID_CURRENCIES_FORMAT,
        KEYS_SYMBOL,
        KEY_SYMBOL,
        METAL_SYMBOL,
        ONE_REF,
    },
};
use serde::{Deserialize, Deserializer};

pub fn metal_deserializer<'de, D>(deserializer: D) -> Result<i32, D::Error>
where
    D: Deserializer<'de>
{
    let float = f32::deserialize(deserializer)?;
    let metal = (float * (ONE_REF as f32)).round() as i32;
    
    Ok(metal)
}

pub fn pluralize<'a>(amount: i32, singular: &'a str, plural: &'a str) -> &'a str {
    if amount == 1 {
        singular
    } else {
        plural
    }
}

pub fn pluralize_float<'a>(amount: f32, singular: &'a str, plural: &'a str) -> &'a str {
    if amount == 1.0 {
        singular
    } else {
        plural
    }
}

pub fn print_float(amount: f32) -> String {
    if amount % 1.0 == 0.0 {
        (amount.round() as i32).to_string()
    } else {
        format!("{:.2}", amount)
    }
}

/// Converts a metal value into its float value.
///
/// # Examples
///
/// ```
/// assert_eq!(0.33, tf2_price::get_metal_float(6));
/// ```
pub fn get_metal_float(value: i32) -> f32 {
    f32::trunc((value as f32 / (ONE_REF as f32)) * 100.0) / 100.0
}

/// Converts a float value into a metal value.
///
/// # Examples
///
/// ```
/// assert_eq!(6, tf2_price::get_metal_from_float(0.33));
/// ```
pub fn get_metal_from_float(value: f32) -> i32 {
    (value * (ONE_REF as f32)).round() as i32
}

pub fn parse_from_string<T>(string: &str) -> Result<(T, i32), &'static str>
where
    T: Default + FromStr + PartialEq
{
    let mut keys = T::default();
    let mut metal = 0;
    
    for element in string.split(", ") {
        let mut element_split = element.split(' ');
        let (
            count_str,
            currency_name,
        ) = (
            element_split.next(),
            element_split.next(),
        );
        
        if count_str.is_none() || currency_name.is_none() || element_split.next().is_some() {
            return Err(INVALID_CURRENCIES_FORMAT);
        }
        
        let (
            count_str,
            currency_name,
        ) = (
            count_str.unwrap(),
            currency_name.unwrap(),
        );
        
        match currency_name {
            KEY_SYMBOL | KEYS_SYMBOL => {
                if let Ok(count) = count_str.parse::<T>() {
                    keys = count;
                } else {
                    return Err("Error parsing key count");
                }
            },
            METAL_SYMBOL => {
                if let Ok(count) = count_str.parse::<f32>() {
                    metal = get_metal_from_float(count);
                } else {
                    return Err("Error parsing metal count");
                }
            },
            _ => {
                return Err(INVALID_CURRENCIES_FORMAT);
            },
        }
    }
    
    if keys == T::default() && metal == 0 {
        return Err("No currencies could be parsed from string");
    }
    
    Ok((keys, metal))
}

pub fn round_metal(metal: i32, rounding: &Rounding) -> i32 {
    if metal == 0 {
        return metal;
    }
    
    match *rounding {
        // No rounding needed if the metal value is an even number.
        Rounding::UpScrap if metal % 2 != 0 => {
            metal + 1
        },
        // No rounding needed if the metal value is an even number.
        Rounding::DownScrap if metal % 2 != 0 => {
            metal - 1
        },
        Rounding::Refined => {
            let value = metal + ONE_REF / 2;
            
            value - (value % ONE_REF)
        },
        Rounding::UpRefined => {
            let remainder = metal % ONE_REF;
            
            if remainder != 0 {
                if metal > 0 {
                    metal - (remainder + -ONE_REF)
                } else {
                    metal - remainder
                }
            } else {
                metal
            }
        },
        Rounding::DownRefined => {
            let remainder = metal % ONE_REF;
            
            if remainder != 0 {
                if metal > 0 {
                    metal - remainder
                } else {
                    metal - (remainder + ONE_REF)
                }
            } else {
                metal
            }
        },
        _ => {
            metal
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scrap;
    
    #[test]
    fn prints_float_rounded_whole_number() {
        assert_eq!("1", print_float(1.0));
    }
    
    #[test]
    fn prints_float_proper_decimal_places() {
        assert_eq!("1.56", print_float(1.55555));
    }
    
    #[test]
    fn converts_from_metal_float() {
        assert_eq!(scrap!(3), get_metal_from_float(0.33));
    }
    
    #[test]
    fn converts_to_metal_float() {
        assert_eq!(0.33, get_metal_float(6));
    }
}