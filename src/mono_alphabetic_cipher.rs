use std::collections::{HashMap, HashSet};

use crate::error::{Error, ErrorKind};

const ALPHABETS: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

pub fn encrypt<S>(plaintext: S, key: S) -> Result<String, Error>
where
    S: AsRef<str>,
{
    if is_valid_key(&key) {
        let key = ALPHABETS
            .chars()
            .zip(key.as_ref().chars())
            .collect::<HashMap<char, char>>();
        let ciphertext = plaintext
            .as_ref()
            .chars()
            .filter_map(|c| key.get(&c.to_ascii_uppercase()))
            .collect::<String>();

        Ok(ciphertext)
    } else {
        Err(Error::new(ErrorKind::InvalidMonoAlphabeticCipherKey))
    }
}

pub fn decrypt<S>(ciphertext: S, key: S) -> Result<String, Error>
where
    S: AsRef<str>,
{
    if is_valid_key(&key) {
        let key = key
            .as_ref()
            .chars()
            .zip(ALPHABETS.chars())
            .collect::<HashMap<char, char>>();
        let plaintext = ciphertext
            .as_ref()
            .chars()
            .filter_map(|c| key.get(&c.to_ascii_uppercase()))
            .collect::<String>();

        Ok(plaintext)
    } else {
        Err(Error::new(ErrorKind::InvalidMonoAlphabeticCipherKey))
    }
}

fn is_valid_key<S>(key: S) -> bool
where
    S: AsRef<str>,
{
    key.as_ref()
        .chars()
        .filter(|&c| ALPHABETS.contains(c.to_ascii_uppercase()))
        .collect::<HashSet<char>>()
        .len()
        == 26
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mono_alphabetic_cipher_encrypt_test() {
        assert_eq!(
            "LTEKTM".to_string(),
            encrypt("SECRET", "AZERTYUIOPQSDFGHJKLMWXCVBN").unwrap(),
        );
    }

    #[test]
    fn mono_alphabetic_cipher_decrypt_test() {
        assert_eq!(
            "SECRET".to_string(),
            decrypt("LTEKTM", "AZERTYUIOPQSDFGHJKLMWXCVBN").unwrap(),
        );
    }
}
