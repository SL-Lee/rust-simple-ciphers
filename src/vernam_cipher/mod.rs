pub mod error;

use error::{Error, ErrorKind};

const ALPHABETS: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

pub fn encrypt<S>(plaintext: S, key: S) -> Result<String, Error>
where
    S: AsRef<str>,
{
    if is_valid_key(&plaintext, &key) {
        let ciphertext = key
            .as_ref()
            .chars()
            .filter_map(|c| ALPHABETS.find(c.to_ascii_uppercase()))
            .zip(
                plaintext
                    .as_ref()
                    .chars()
                    .filter_map(|c| ALPHABETS.find(c.to_ascii_uppercase())),
            )
            .map(|(i, j)| ALPHABETS.chars().nth((i + j) % 26).unwrap())
            .collect::<String>();

        Ok(ciphertext)
    } else {
        Err(Error::new(ErrorKind::InvalidKey))
    }
}

pub fn decrypt<S>(ciphertext: S, key: S) -> Result<String, Error>
where
    S: AsRef<str>,
{
    if is_valid_key(&ciphertext, &key) {
        let plaintext = key
            .as_ref()
            .chars()
            .filter_map(|c| ALPHABETS.find(c.to_ascii_uppercase()))
            .zip(
                ciphertext
                    .as_ref()
                    .chars()
                    .filter_map(|c| ALPHABETS.find(c.to_ascii_uppercase())),
            )
            .map(|(i, j)| {
                ALPHABETS
                    .chars()
                    .nth((((j as isize - i as isize) + 26) % 26) as usize)
                    .unwrap()
            })
            .collect::<String>();

        Ok(plaintext)
    } else {
        Err(Error::new(ErrorKind::InvalidKey))
    }
}

fn is_valid_key<S>(plaintext: S, key: S) -> bool
where
    S: AsRef<str>,
{
    plaintext
        .as_ref()
        .chars()
        .filter(|&c| ALPHABETS.contains(c.to_ascii_uppercase()))
        .count()
        == key
            .as_ref()
            .chars()
            .filter(|&c| ALPHABETS.contains(c.to_ascii_uppercase()))
            .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vernam_cipher_encrypt_test() {
        assert_eq!("DSPXIK".to_string(), encrypt("SECRET", "LONGER").unwrap());
    }

    #[test]
    fn vernam_cipher_decrypt_test() {
        assert_eq!("SECRET".to_string(), decrypt("DSPXIK", "LONGER").unwrap());
    }
}
