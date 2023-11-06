const MAT_SIZE: usize = 256;
const ALPHABET_MAT: [[char; MAT_SIZE]; MAT_SIZE] = generate_alphabet_matrix();

const fn generate_alphabet_matrix() -> [[char; MAT_SIZE]; MAT_SIZE] {
    let mut mat = [['0'; MAT_SIZE]; MAT_SIZE];
    let mut i = 0;
    while i < MAT_SIZE {
        let mut j = 0;
        while j < MAT_SIZE {
            mat[i][j] = ((i + j) % MAT_SIZE) as u8 as char;
            j += 1;
        }
        i += 1;
    }
    mat
}

// TODO: Return errors from functions

pub fn vigenere_cipher(plain_text: &str, key: &str) -> String {
    let key_chars = repeat_key(key, plain_text.len());
    let mut cipher_text = String::new();
    for (i, c) in plain_text.chars().enumerate() {
        cipher_text.push(ALPHABET_MAT[key_chars[i] as usize][c as usize]); // TODO: Add % 256
    }
    cipher_text
}

pub fn vigenere_decipher(cipher_text: &str, key: &str) -> String {
    let key_chars = repeat_key(key, cipher_text.len());
    let mut plain_text = String::new();
    for (i, &kc) in key_chars.iter().enumerate() {
        let row = kc as usize;
        if let Some(tc) = cipher_text.chars().nth(i) {
            for (i, &ac) in ALPHABET_MAT[row].iter().enumerate() {
                if ac == tc {
                    plain_text.push(ALPHABET_MAT[0][i]);
                }
            }
        }
    }
    plain_text
}

fn repeat_key(key: &str, len: usize) -> Vec<char> {
    let mut chars: Vec<char> = key.chars().collect();
    if key.len() >= len {
        return chars;
    }
    for i in 0..(len - key.len()) {
        chars.push(chars[i % key.len()]);
    }
    chars
}
