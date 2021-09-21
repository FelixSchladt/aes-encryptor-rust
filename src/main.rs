mod encrypt;
mod file_processor;
mod password;
use std::env;

fn wrong_args() {
    println!(
        "encryptor: missing option and file operand\nTry \'encryptor -h\' for more information."
    );
    std::process::exit(1);
}

fn argparse() {
    let path = String::from("./");

    let mut args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        if args.len() == 2 {
            if &args[1][..] != "-h" && &args[1][..] != "--help" {
                wrong_args();
            }
        } else {
            wrong_args();
        }
    }
    args.remove(0);
    match &args[0][..] {
        "-e" => {
            args.remove(0);
            file_processor::check_only_files(&args);
            let counter = encrypt::encrypt_files(args, password::password_with_confirmation());
            println!(
                "Succesfully encrpyted {} file{}.",
                counter,
                if counter > 1 { "s" } else { "" }
            );
        }
        "-ef" => {
            args.remove(0);
            let counter = encrypt::encrypt_files(
                file_processor::get_files(args, path),
                password::password_with_confirmation(),
            );
            println!(
                "Succesfully encrpyted {} file{}.",
                counter,
                if counter > 1 { "s" } else { "" }
            );
        }
        "-d" => {
            args.remove(0);
            file_processor::check_only_files(&args);
            let counter = encrypt::decrypt_files(args, password::password());
            println!(
                "Succesfully decrpyted {} file{}.",
                counter,
                if counter > 1 { "s" } else { "" }
            );
        }
        "-df" => {
            args.remove(0);
            let counter =
                encrypt::decrypt_files(file_processor::get_files(args, path), password::password());
            println!(
                "Succesfully decrpyted {} file{}.",
                counter,
                if counter > 1 { "s" } else { "" }
            );
        }
        "-h" | "--help" => {
            println!("USAGE:        encrypt -[OPTIONS] FILE FILE...\n ");
            println!("     -h       Show this help message");
            println!("     -d       decrypt file or filesn");
            println!("     -df      decrypt with folder support");
            println!("     -e       encrypt file or files");
            println!("     -ef      encrypt with folder support\n");
        }
        _ => {
            wrong_args();
        }
    }
}

fn main() {
    argparse();
}
