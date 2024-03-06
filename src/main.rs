use md5;
use aes::utils::*;
use std::fs::File;
use std::io::prelude::*;
use rand::Rng;

fn main() {
    let input_path = "/home/tulitanir/aes/target/1.png";
    let key_word = "key";
    let output_path = "/home/tulitanir/aes/target/encrypted.png";

    let input_path2 = "/home/tulitanir/aes/target/encrypted.png";
    let output_path2 = "/home/tulitanir/aes/target/2.png";

    encrypt_file(input_path, key_word, output_path);
    decrypt_file(input_path2, key_word, output_path2);
}

fn encrypt_file(input_path: &str, key_word: &str, output_path: &str) {
    let key = md5::compute(key_word);
    let mut file = File::open(input_path).unwrap();
    let mut buffer = [0u8; 16];

    let mut encrypted_bytes: Vec<u8> = Vec::new();

    loop {
        let bytes_read = file.read(&mut buffer).unwrap();

        if bytes_read < 16 {
            let tmp = buffer;
    
            buffer[0] = bytes_read as u8;
            for i in 1..(bytes_read + 1) as usize {
                buffer[i] = tmp[i - 1];
            }

            let mut rng = rand::thread_rng();
    
            for i in (bytes_read + 1) as usize..16 {
                buffer[i] = rng.gen();
            }

            encrypted_bytes.append(&mut encrypt(&buffer, &key).to_vec());
            break;
        }

        encrypted_bytes.append(&mut encrypt(&buffer, &key).to_vec());
    }

    let mut res = File::create(output_path).unwrap(); 
    res.write_all(&encrypted_bytes).unwrap();
}

fn decrypt_file(input_path: &str, key_word: &str, output_path: &str) {
    let key = md5::compute(key_word);
    let mut file = File::open(input_path).unwrap();
    let mut buffer = [0u8; 16];

    let mut encrypted_bytes: Vec<u8> = Vec::new();

    loop {
        let bytes_read = file.read(&mut buffer).unwrap();

        if bytes_read < 16 {
        //     let tmp = buffer;
    
        //     let num_bytes = buffer[0];
        //     for i in 1..(num_bytes + 1) as usize {
        //         buffer[i] = tmp[i - 1];
        //     }

        //     let mut rng = rand::thread_rng();
    
        //     for i in (num_bytes + 1) as usize..16 {
        //         buffer[i] = rng.gen();
        //     }

        //     encrypted_bytes.append(&mut decrypt(&buffer, &key).to_vec());
            break;
        }

        encrypted_bytes.append(&mut decrypt(&buffer, &key).to_vec());
    }

    let mut res = File::create(output_path).unwrap(); 
    res.write_all(&encrypted_bytes).unwrap();
}
