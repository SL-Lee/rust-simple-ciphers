pub mod error;

use error::{Error, ErrorKind};

const ALPHABETS: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

pub fn encrypt<S>(plaintext: S, key: usize) -> Result<String, Error>
where
    S: AsRef<str>,
{
    if (1..=25).contains(&key) {
        let ciphertext = plaintext
            .as_ref()
            .chars()
            .map(|c| c.to_ascii_uppercase())
            .filter_map(|c| ALPHABETS.find(c))
            .map(|index| ALPHABETS.chars().nth((index + key) % 26).unwrap())
            .collect::<String>();

        Ok(ciphertext)
    } else {
        Err(Error::new(ErrorKind::InvalidKey))
    }
}

pub fn decrypt<S>(ciphertext: S, key: usize) -> Result<String, Error>
where
    S: AsRef<str>,
{
    if (1..=25).contains(&key) {
        let plaintext = ciphertext
            .as_ref()
            .chars()
            .map(|c| c.to_ascii_uppercase())
            .filter_map(|c| ALPHABETS.find(c))
            .map(|index| {
                ALPHABETS
                    .chars()
                    .nth((((index as isize - key as isize) + 26) % 26) as usize)
                    .unwrap()
            })
            .collect::<String>();

        Ok(plaintext)
    } else {
        Err(Error::new(ErrorKind::InvalidKey))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn caesar_cipher_encrypt_test() {
        assert_eq!("VHFUHW".to_string(), encrypt("SECRET", 3).unwrap());
    }

    #[test]
    fn caesar_cipher_decrypt_test() {
        assert_eq!("SECRET".to_string(), decrypt("VHFUHW", 3).unwrap());
    }
}
