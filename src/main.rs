use md5;
use aes::utils::*;

fn main() {
    let bytes = b"Hello, World!!!!";
    // process input message
    let key = md5::compute(b"key");

    let res = encrypt(bytes, &key);

    println!("{}", String::from_utf8(bytes.to_vec()).unwrap());

    println!("{}", String::from_utf8(decrypt(&res, &key).to_vec()).unwrap());

    println!("{:?}", decrypt(&res, &key));

}
