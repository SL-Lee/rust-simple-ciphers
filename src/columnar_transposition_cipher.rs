use crate::error::{Error, ErrorKind};

pub fn encrypt<S>(plaintext: S, key: S) -> Result<String, Error>
where
    S: AsRef<str>,
{
    if is_valid_key(&key) {
        let mut key = key
            .as_ref()
            .chars()
            .zip(0..key.as_ref().len())
            .collect::<Vec<(char, usize)>>();
        key.sort_by_key(|&(i, _)| i);
        let key = key.iter().map(|&(_, j)| j).collect::<Vec<usize>>();
        let mut plaintext = plaintext.as_ref().as_bytes().to_vec();

        while plaintext.len() % key.len() != 0 {
            plaintext.push(0);
        }

        let plaintext =
            plaintext.chunks_exact(key.len()).collect::<Vec<&[u8]>>();
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
    } else {
        Err(Error::new(ErrorKind::InvalidColumnarTranspositionCipherKey))
    }
}

pub fn decrypt<S>(ciphertext: S, key: S) -> Result<String, Error>
where
    S: AsRef<str>,
{
    if is_valid_key(&key) {
        let ciphertext = ciphertext.as_ref();
        let key = key
            .as_ref()
            .chars()
            .map(|c| (c.to_digit(10).unwrap() - 1) as usize)
            .collect::<Vec<usize>>();
        let remainder_range =
            (0..ciphertext.len() % key.len()).collect::<Vec<usize>>();
        let full_rows = ciphertext.len() / key.len();
        let total_rows =
            full_rows + if remainder_range.is_empty() { 0 } else { 1 };
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
    } else {
        Err(Error::new(ErrorKind::InvalidColumnarTranspositionCipherKey))
    }
}

fn is_valid_key<S>(key: S) -> bool
where
    S: AsRef<str>,
{
    let key = key.as_ref();
    let required_digits = (1..key.len() + 1).collect::<Vec<usize>>();
    key.chars().all(|c| c.is_ascii_digit())
        && key
            .chars()
            .map(|c| c.to_digit(10).unwrap() as usize)
            .all(|d| required_digits.contains(&d))
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
