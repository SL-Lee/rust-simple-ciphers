use crate::error::{Error, ErrorKind};

const ALPHABETS: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

pub fn encrypt<S>(plaintext: S, key: S) -> Result<String, Error>
where
    S: AsRef<str>,
{
    let key = validate_key(&plaintext, &key)?;
    let ciphertext = key
        .as_ref()
        .chars()
        .filter_map(|c| ALPHABETS.find(c.to_ascii_uppercase()))
        .zip(plaintext.as_ref().chars().filter_map(|c| ALPHABETS.find(c.to_ascii_uppercase())))
        .map(|(i, j)| ALPHABETS.chars().nth((i + j) % 26).unwrap())
        .collect::<String>();

    Ok(ciphertext)
}

pub fn decrypt<S>(ciphertext: S, key: S) -> Result<String, Error>
where
    S: AsRef<str>,
{
    let key = validate_key(&ciphertext, &key)?;
    let plaintext = key
        .as_ref()
        .chars()
        .filter_map(|c| ALPHABETS.find(c.to_ascii_uppercase()))
        .zip(ciphertext.as_ref().chars().filter_map(|c| ALPHABETS.find(c.to_ascii_uppercase())))
        .map(|(i, j)| {
            ALPHABETS.chars().nth((((j as isize - i as isize) + 26) % 26) as usize).unwrap()
        })
        .collect::<String>();

    Ok(plaintext)
}

pub fn validate_key<S>(input: S, key: S) -> Result<S, Error>
where
    S: AsRef<str>,
{
    let plaintext = input.as_ref();

    if plaintext.chars().filter(|&c| ALPHABETS.contains(c.to_ascii_uppercase())).count()
        == key.as_ref().chars().filter(|&c| ALPHABETS.contains(c.to_ascii_uppercase())).count()
    {
        Ok(key)
    } else {
        Err(Error::new(ErrorKind::InvalidVernamCipherKey))
    }
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
