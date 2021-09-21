//use dialoguer::Password;

use std::io::Write;
use std::io::stdout;
use console::Term;
//use zeroize::Zeroizing;

pub fn password_with_confirmation() -> String {
    /*let password = Password::new().with_prompt("Password")
        .with_confirmation("Confirm password", "Passwords mismatching")
        .interact().unwrap();
    return password;
    */
    let password = password_prompt("Password");
    if password == password_prompt("Confirm password") {
        return password;
    } else {
        eprintln!("error: passwords do not match");
        std::process::exit(1);
    }
}

pub fn password() -> String {
    return password_prompt("Password");
}

pub fn password_prompt(prompt: &str) -> String {
    //return Password::new().with_prompt("Password").interact().unwrap();
    print!("{}: ", prompt);
    stdout().flush().unwrap();
    let terminal = Term::stdout();
    let password = Term::read_secure_line(&terminal).unwrap();
    terminal.clear_last_lines(1).unwrap();
    return password;
}
