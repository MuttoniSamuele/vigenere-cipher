mod vigenere;

fn main() {
    // TODO: Create UI
    let plain_text = "Hello World!";
    let key = "key";
    let cipher_text = vigenere::vigenere_cipher(plain_text, key);
    println!("{}", cipher_text);
    let deciphered_text = vigenere::vigenere_decipher(&cipher_text, key);
    println!("{}", deciphered_text);
}
