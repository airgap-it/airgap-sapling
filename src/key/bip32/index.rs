use regex::Regex;
use zcash_primitives::zip32::ChildIndex;

use crate::common::errors::{CausedBy, SaplingError};
use crate::common::utils::regex_utils::{contains_chars_re, not_contains_chars_re};

use super::errors::Bip32IndexError;

const MASK_HARD_DERIVATION: u32 = 0x8000_0000;
const MASK_SOFT_DERIVATION: u32 = 0x0000_0000;

const VALID_INDEX_CHARACTERS_RE: &str = "0-9";
const VALID_IS_HARD_CHARACTERS_RE: &str = "'h";

pub type Bip32Index = ChildIndex;

pub fn create_index(index: &str) -> Result<Bip32Index, SaplingError> {
    parse_index(index).map_err(SaplingError::caused_by)
}

fn parse_index(index: &str) -> Result<Bip32Index, Bip32IndexError> {
    assert_index_non_empty(index)?;
    assert_index_valid(index)?;

    let is_hard = is_hard_index(index);
    let index_end = get_index_end(index, is_hard);

    let index = &index[..index_end];
    let index = index.parse::<u32>().or_else(|_| Err(Bip32IndexError::ParseError))?;
    let index = mask_index(index, is_hard);

    Ok(Bip32Index::from_index(index))
}

fn assert_index_non_empty(index: &str) -> Result<(), Bip32IndexError> {
    if index.is_empty() {
        Err(Bip32IndexError::Empty)
    } else {
        Ok(())
    }
}

fn assert_index_valid(index: &str) -> Result<(), Bip32IndexError> {
    let invalid_regex = {
        let invalid_re = not_contains_chars_re(&[VALID_INDEX_CHARACTERS_RE, VALID_IS_HARD_CHARACTERS_RE]);
        Regex::new(&invalid_re).expect("could not check bip32 index, invalid regular expression")
    };

    if invalid_regex.is_match(index) {
        let invalid: Vec<String> = invalid_regex.find_iter(index).map(|m| index[m.start()..m.end()].to_string()).collect();
        Err(Bip32IndexError::InvalidCharacter(invalid))
    } else {
        Ok(())
    }
}

fn is_hard_index(index: &str) -> bool {
    let is_hard_regex = {
        let is_hard_re = contains_chars_re(&[VALID_IS_HARD_CHARACTERS_RE]);
        Regex::new(&is_hard_re).expect("could not create bip32 index, invalid `is_hard` regular expression")
    };

    is_hard_regex.is_match(index)
}

fn get_index_end(index: &str, is_hard: bool) -> usize {
    let len = index.len();
    if is_hard {
        len - 1
    } else {
        len
    }
}

fn mask_index(index: u32, is_hard: bool) -> u32 {
    let mask = if is_hard {
        MASK_HARD_DERIVATION
    } else {
        MASK_SOFT_DERIVATION
    };

    index | mask
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
   fn creates_bip32_index_from_valid_string() {
       let test_data = vec![
           ("44'", Bip32Index::Hardened(44)),
           ("0h", Bip32Index::Hardened(0)),
           ("0", Bip32Index::NonHardened(0)),
       ];

       let actual_expected = test_data.iter()
           .map(|(i, expected)| {
               let actual = create_index(i).unwrap();

               (actual, expected)
           });

       for (actual, expected) in actual_expected {
           assert_eq!(actual, *expected);
       }
   }

   #[test]
   fn fails_with_empty_index_error_if_empty_string() {
      let empty = create_index("").unwrap_err();

      assert_eq!(empty, SaplingError::caused_by(Bip32IndexError::Empty));
   }
   
   #[test]
   fn fails_with_invalid_character_error_if_string_contains_illegal_characters() {
       let test_data = vec![
           ("-44", SaplingError::caused_by(Bip32IndexError::InvalidCharacter(vec!["-".to_string()]))),
           ("abc", SaplingError::caused_by(Bip32IndexError::InvalidCharacter(vec!["a".to_string(), "b".to_string(), "c".to_string()]))),
           ("1hg", SaplingError::caused_by(Bip32IndexError::InvalidCharacter(vec!["g".to_string()]))),
       ];

       let actual_expected = test_data.iter()
           .map(|(i, err)| {
               let actual = create_index(i).unwrap_err();

               (actual, err)
           });

       for (actual, expected) in actual_expected {
           assert_eq!(actual, *expected);
       }
   }

    #[test]
    fn fails_with_parsing_error_if_index_is_too_big() {
        let too_big = create_index(&(std::u64::MAX).to_string()[..]).unwrap_err();

        assert_eq!(too_big, SaplingError::caused_by(Bip32IndexError::ParseError));
    }
}