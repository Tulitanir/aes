use aes::utils::*;
use clap::{command, Arg, ArgAction};


fn main() {
    let match_result = command!()
        .about("A simple CLI tool for AES-128 encryption/decryption")
        .arg(
            Arg::new("input")
                .short('i')
                .long("input")
                .required(true)
        )
        .arg(
            Arg::new("output")
            .short('o')
            .long("output")
            .required(true)
        )
        .arg(
            Arg::new("key")
            .short('k')
            .long("key")
            .required(true)
        )
        .arg(
            Arg::new("encrypt")
            .short('e')
            .long("encrypt")
            .action(ArgAction::SetTrue)
            .required(true)
            .conflicts_with("decrypt")
        )
        .arg(
            Arg::new("decrypt")
            .short('d')
            .long("decrypt")
            .action(ArgAction::SetTrue)
            .required(true)
            .conflicts_with("encrypt")
        )
        .get_matches();


    let Some(input_path) = match_result.get_one::<String>("input") else {
        panic!("Input wasn't been specified");
    };
    let Some(output_path) = match_result.get_one::<String>("output") else {
        panic!("Output was't been specified")
    };
    let Some(key_word) = match_result.get_one::<String>("key") else {
        panic!("Key was't been specified")
    };

    if match_result.get_flag("encrypt") {
        encrypt_file(input_path, key_word, output_path);
    }

    if match_result.get_flag("decrypt") {
        decrypt_file(input_path, key_word, output_path);
    }
}
