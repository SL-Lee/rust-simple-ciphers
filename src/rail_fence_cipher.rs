use crate::error::{Error, ErrorKind};

pub fn encrypt<S>(plaintext: S, key: usize) -> Result<String, Error>
where
    S: AsRef<str>,
{
    if key > 1 {
        let plaintext = plaintext.as_ref();
        let mut ciphertext = vec![String::new(); key];
        let sequence = generate_sequence(plaintext.len(), key);

        for (i, &j) in sequence.iter().enumerate() {
            ciphertext[j].push(plaintext.chars().nth(i).unwrap());
        }

        Ok(ciphertext.join(""))
    } else {
        Err(Error::new(ErrorKind::InvalidRailFenceCipherKey))
    }
}

pub fn decrypt<S>(ciphertext: S, key: usize) -> Result<String, Error>
where
    S: AsRef<str>,
{
    if key > 1 {
        let ciphertext = ciphertext.as_ref();
        let mut rails = vec![Vec::new(); key];
        let sequence = generate_sequence(ciphertext.len(), key);

        for &i in &sequence {
            for (j, rail) in rails.iter_mut().enumerate().take(key) {
                rail.push(if i == j { 1 } else { 0 });
            }
        }

        let mut count = 0;

        for rail in rails.iter_mut().take(key) {
            while let Some(index) = rail.iter().position(|&e| e == 1) {
                rail[index] = ciphertext.chars().nth(count).unwrap() as u8;
                count += 1;
            }
        }

        let mut plaintext = String::new();

        for (i, &j) in sequence.iter().enumerate() {
            plaintext.push(rails[j][i] as char);
        }

        Ok(plaintext)
    } else {
        Err(Error::new(ErrorKind::InvalidRailFenceCipherKey))
    }
}

fn generate_sequence(length: usize, rows: usize) -> Vec<usize> {
    let mut sequence = Vec::with_capacity(length);
    let mut row: isize = 0;
    let mut direction: isize = 1;

    for _ in 0..length {
        if row + 1 == rows as isize {
            direction = -1;
        } else if row == 0 {
            direction = 1;
        }

        sequence.push(row as usize);
        row += direction;
    }

    sequence
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rail_fence_encrypt_test() {
        assert_eq!("SEERTC".to_string(), encrypt("SECRET", 3).unwrap());
    }

    #[test]
    fn rail_fence_decrypt_test() {
        assert_eq!("SECRET".to_string(), decrypt("SEERTC", 3).unwrap());
    }
}
