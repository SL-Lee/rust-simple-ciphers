use crate::error::{Error, ErrorKind};

pub fn encrypt<S>(plaintext: S, key: S) -> Result<String, Error>
where
    S: AsRef<str>,
{
    let key = validate_encryption_key(key)?;
    let mut plaintext = plaintext.as_ref().as_bytes().to_vec();

    while plaintext.len() % key.len() != 0 {
        plaintext.push(0);
    }

    let plaintext = plaintext.chunks_exact(key.len()).collect::<Vec<&[u8]>>();
    let mut ciphertext = String::new();

    for i in 0..key.len() {
        for j in &plaintext {
            let index = key.iter().position(|&d| d == i).unwrap();

            if j[index] != 0 {
                ciphertext.push(j[index] as char);
            }
        }
    }

    Ok(ciphertext)
}

pub fn decrypt<S>(ciphertext: S, key: S) -> Result<String, Error>
where
    S: AsRef<str>,
{
    let key = validate_decryption_key(key)?;
    let ciphertext = ciphertext.as_ref();
    let remainder_range =
        (0..ciphertext.len() % key.len()).collect::<Vec<usize>>();
    let full_rows = ciphertext.len() / key.len();
    let total_rows = full_rows + if remainder_range.is_empty() { 0 } else { 1 };
    let mut columns = vec![String::new(); key.len()];
    let mut count = 0;

    for &i in &key {
        columns[i].push_str(&ciphertext[count..count + full_rows]);
        count += full_rows;

        if remainder_range.contains(&i) {
            columns[i].push(ciphertext.chars().nth(count).unwrap());
            count += 1;
        }
    }

    let mut plaintext = String::new();

    for i in 0..total_rows {
        for j in &columns {
            if i < j.len() {
                plaintext.push(j.chars().nth(i).unwrap());
            }
        }
    }

    Ok(plaintext)
}

pub fn validate_encryption_key<S>(key: S) -> Result<Vec<usize>, Error>
where
    S: AsRef<str>,
{
    let key = key.as_ref();
    let required_digits = (1..key.len() + 1).collect::<Vec<usize>>();

    if key.chars().all(|c| {
        c.is_ascii_digit()
            && required_digits.contains(&(c.to_digit(10).unwrap() as usize))
    }) {
        let mut key = key
            .chars()
            .zip(0..key.len())
            .collect::<Vec<(char, usize)>>();
        key.sort_by_key(|&(i, _)| i);
        Ok(key.iter().map(|&(_, j)| j).collect::<Vec<usize>>())
    } else {
        Err(Error::new(ErrorKind::InvalidColumnarTranspositionCipherKey))
    }
}

pub fn validate_decryption_key<S>(key: S) -> Result<Vec<usize>, Error>
where
    S: AsRef<str>,
{
    let key = key.as_ref();
    let required_digits = (1..key.len() + 1).collect::<Vec<usize>>();

    if key.chars().all(|c| {
        c.is_ascii_digit()
            && required_digits.contains(&(c.to_digit(10).unwrap() as usize))
    }) {
        Ok(key
            .chars()
            .map(|c| (c.to_digit(10).unwrap() - 1) as usize)
            .collect::<Vec<usize>>())
    } else {
        Err(Error::new(ErrorKind::InvalidColumnarTranspositionCipherKey))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn columnar_transposition_encrypt_test() {
        assert_eq!("ETSERC".to_string(), encrypt("SECRET", "2143").unwrap());
    }

    #[test]
    fn columnar_transposition_decrypt_test() {
        assert_eq!("SECRET".to_string(), decrypt("ETSERC", "2143").unwrap());
    }
}
