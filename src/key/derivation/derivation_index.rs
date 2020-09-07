use regex::Regex;
use zcash_primitives::zip32::ChildIndex;

use crate::utils::regex_utils::{contains_chars_re, not_contains_chars_re};
use super::errors::DerivationPathError;

const MASK_HARD_DERIVATION: u32 = 0x8000_0000;
const MASK_SOFT_DERIVATION: u32 = 0x0000_0000;

const VALID_INDEX_CHARACTERS_RE: &str = "0-9";
const VALID_IS_HARD_CHARACTERS_RE: &str = "'h";

pub type DerivationIndex = ChildIndex;

pub fn create_derivation_index(index: &str) -> Result<DerivationIndex, DerivationPathError> {
    assert_index_non_empty(index)?;
    assert_index_valid(index)?;

    let is_hard = is_hard_index(index);
    let index_end = get_index_end(index, is_hard);

    let index = &index[..index_end];
    let index = index.parse::<u32>().or_else(|_| Err(DerivationPathError::unknown("could not parse derivation index")))?;
    let index = mask_index(index, is_hard);

    Ok(DerivationIndex::from_index(index))
}

fn assert_index_non_empty(index: &str) -> Result<(), DerivationPathError> {
    if index.is_empty() {
        Err(DerivationPathError::EmptyIndex)
    } else {
        Ok(())
    }
}

fn assert_index_valid(index: &str) -> Result<(), DerivationPathError> {
    let invalid_regex = {
        let invalid_re = not_contains_chars_re(&[VALID_INDEX_CHARACTERS_RE, VALID_IS_HARD_CHARACTERS_RE]);
        Regex::new(&invalid_re).expect("could not check derivation index, invalid regular expression")
    };

    if invalid_regex.is_match(index) {
        let invalid = invalid_regex.find_iter(index).map(|m| &index[m.start()..m.end()]).collect();
        Err(DerivationPathError::invalid_character(invalid))
    } else {
        Ok(())
    }
}

fn is_hard_index(index: &str) -> bool {
    let is_hard_regex = {
        let is_hard_re = contains_chars_re(&[VALID_IS_HARD_CHARACTERS_RE]);
        Regex::new(&is_hard_re).expect("could not create derivation index, invalid `is_hard` regular expression")
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
mod test {
  use super::*;

   #[test]
   fn creates_derivation_index_from_valid_string() {
       let test_data = vec![
           ("44'", DerivationIndex::Hardened(44)),
           ("0h", DerivationIndex::Hardened(0)),
           ("0", DerivationIndex::NonHardened(0)),
       ];

       let actual_expected = test_data.iter()
           .map(|(i, expected)| {
               let actual = create_derivation_index(i).unwrap();

               (actual, expected)
           });

       for (actual, expected) in actual_expected {
           assert_eq!(actual, *expected);
       }
   }

   #[test]
   fn fails_with_empty_index_error_if_empty_string() {
      let empty = create_derivation_index("");

      assert_eq!(empty, Err(DerivationPathError::EmptyIndex));
   }
   
   #[test]
   fn fails_with_invalid_character_error_if_string_contains_illegal_characters() {
       let test_data = vec![
           ("-44", DerivationPathError::invalid_character(vec!["-"])),
           ("abc", DerivationPathError::invalid_character(vec!["a", "b", "c"])),
           ("1hg", DerivationPathError::invalid_character(vec!["g"])),
       ];

       let actual_expected = test_data.iter()
           .map(|(i, err)| {
               let actual = create_derivation_index(i).unwrap_err();

               (actual, err)
           });

       for (actual, expected) in actual_expected {
           assert_eq!(actual, *expected);
       }
   }

    #[test]
    fn fails_with_parsing_error_if_index_is_too_big() {
        let too_big = create_derivation_index(&(std::u64::MAX).to_string()[..]);

        assert_eq!(too_big, Err(DerivationPathError::unknown("could not parse derivation index")));
    }
}