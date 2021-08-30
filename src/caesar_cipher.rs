use crate::error::{Error, ErrorKind};

const ALPHABETS: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

pub fn encrypt<S>(plaintext: S, key: usize) -> Result<String, Error>
where
    S: AsRef<str>,
{
    let key = validate_key(key)?;
    let ciphertext = plaintext
        .as_ref()
        .chars()
        .filter_map(|c| match ALPHABETS.find(c.to_ascii_uppercase()) {
            Some(index) => ALPHABETS.chars().nth((index + key) % 26),
            None => Some(c),
        })
        .collect::<String>();

    Ok(ciphertext)
}

pub fn decrypt<S>(ciphertext: S, key: usize) -> Result<String, Error>
where
    S: AsRef<str>,
{
    let key = validate_key(key)?;
    let plaintext = ciphertext
        .as_ref()
        .chars()
        .filter_map(|c| match ALPHABETS.find(c.to_ascii_uppercase()) {
            Some(index) => ALPHABETS
                .chars()
                .nth((((index as isize - key as isize) + 26) % 26) as usize),
            None => Some(c),
        })
        .collect::<String>();

    Ok(plaintext)
}

pub fn validate_key(key: usize) -> Result<usize, Error> {
    match key {
        1..=25 => Ok(key),
        _ => Err(Error::new(ErrorKind::InvalidCaesarCipherKey)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn caesar_cipher_encrypt_test() {
        assert_eq!(
            "VHFUHW WHAW.".to_string(),
            encrypt("SECRET TEXT.", 3).unwrap()
        );
    }

    #[test]
    fn caesar_cipher_decrypt_test() {
        assert_eq!(
            "SECRET TEXT.".to_string(),
            decrypt("VHFUHW WHAW.", 3).unwrap()
        );
    }
}
