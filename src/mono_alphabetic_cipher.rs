use std::collections::{HashMap, HashSet};

use crate::error::{Error, ErrorKind};

const ALPHABETS: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

pub fn encrypt<S>(plaintext: S, key: S) -> Result<String, Error>
where
    S: AsRef<str>,
{
    let key = validate_encryption_key(key)?;
    let ciphertext = plaintext
        .as_ref()
        .chars()
        .filter_map(|c| key.get(&c.to_ascii_uppercase()))
        .collect::<String>();

    Ok(ciphertext)
}

pub fn decrypt<S>(ciphertext: S, key: S) -> Result<String, Error>
where
    S: AsRef<str>,
{
    let key = validate_decryption_key(key)?;
    let plaintext = ciphertext
        .as_ref()
        .chars()
        .filter_map(|c| key.get(&c.to_ascii_uppercase()))
        .collect::<String>();

    Ok(plaintext)
}

pub fn validate_encryption_key<S>(key: S) -> Result<HashMap<char, char>, Error>
where
    S: AsRef<str>,
{
    let key = key.as_ref();

    if key
        .chars()
        .filter(|&c| ALPHABETS.contains(c.to_ascii_uppercase()))
        .collect::<HashSet<char>>()
        .len()
        == 26
    {
        Ok(ALPHABETS
            .chars()
            .zip(key.chars())
            .collect::<HashMap<char, char>>())
    } else {
        Err(Error::new(ErrorKind::InvalidMonoAlphabeticCipherKey))
    }
}

pub fn validate_decryption_key<S>(key: S) -> Result<HashMap<char, char>, Error>
where
    S: AsRef<str>,
{
    let key = key.as_ref();

    if key
        .chars()
        .filter(|&c| ALPHABETS.contains(c.to_ascii_uppercase()))
        .collect::<HashSet<char>>()
        .len()
        == 26
    {
        Ok(key
            .chars()
            .zip(ALPHABETS.chars())
            .collect::<HashMap<char, char>>())
    } else {
        Err(Error::new(ErrorKind::InvalidMonoAlphabeticCipherKey))
    }
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
