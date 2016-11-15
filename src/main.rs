#[macro_use]
extern crate clap;
extern crate ultra;

use ultra::enigma::Enigma;
use ultra::decrypt::decrypt;

fn main() {
    let app = clap_app!(ultra =>
        (version: crate_version!())
        (@setting ArgRequiredElseHelp)
        (@setting ColoredHelp)
        (@arg decrypt: --decrypt -d "Decrypt a given piece of ciphertext")
        (@arg ROTORS: --rotor -w +takes_value "Rotor order (default: \"123\")")
        (@arg KEY: --key -k +takes_value "Key settings (default: \"AAA\")")
        (@arg RING: --ring -r +takes_value "Ring settings (default: \"AAA\")")
        (@arg PLUGBOARD: --plugboard -p +takes_value "Plugboard settings (default: \"\")")
        (@arg MESSAGE: +required "The message to encrypt/decrypt")
    );

    let matches = app.get_matches();
    let msg = matches.value_of("MESSAGE").unwrap();

    if matches.is_present("decrypt") {
        let (plaintext, key, ring, rotor) = decrypt(msg);
        println!("{}", plaintext);
        println!("(Key Setting: {}, Ring Setting: {}, Rotors: {})", key, ring, rotor);
        return;
    }

    let rotors = matches.value_of("ROTORS").unwrap_or("123");
    let key = matches.value_of("KEY").unwrap_or("AAA");
    let ring = matches.value_of("RING").unwrap_or("AAA");
    let plugboard = matches.value_of("PLUGBOARD").unwrap_or("");

    let mut enigma = Enigma::new(rotors, key, ring, 'B', plugboard);
    println!("{}", enigma.encrypt(msg))
}
