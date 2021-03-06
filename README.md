# ultra [![Build Status][Travis Badge]][Build Status] [![crates.io][crates.io Badge]][crates.io] [![docs.rs][docs.rs Badge]][docs.rs] [![License][License Badge]](LICENSE)

Cryptanalysis of the Enigma in Rust.


## Installation

`ultra` can be installed from crates.io using Cargo:

```
$ cargo install ultra
```


## Usage

Encrypt a message with rotors `1-4-2`, key setting `DOG`, and ring setting `CAT`:

```bash
$ ultra --rotor=142 --key=DOG --ring=CAT ${message}
```

Encrypt a message using random Enigma settings:

```bash
$ ultra --randomize ${message}
```

Attempt to decrypt a given piece of ciphertext:

```bash
$ ultra --decrypt ${message}
```

> **Note**: Decryption relies on quadgram frequencies to infer the original
Enigma machine settings; as a result, it is very likely that short messages
will be decrypted incorrectly. Additionally, decryption will not work for
messages that were encrypted with any plugboard plugs active.


## References

This project's quadgram data and decryption algorithm is based on
[James Lyons'] articles about the Enigma machine.


## License

`ultra` is licensed under the [MIT License](LICENSE).


[Travis Badge]: https://travis-ci.org/iKevinY/ultra.svg?branch=master
[Build Status]: https://travis-ci.org/iKevinY/ultra
[crates.io Badge]: https://img.shields.io/crates/v/ultra.svg
[crates.io]: https://crates.io/crates/ultra
[docs.rs Badge]: https://docs.rs/ultra/badge.svg
[docs.rs]: https://docs.rs/ultra
[License Badge]: https://img.shields.io/crates/l/ultra.svg

[James Lyons']: http://practicalcryptography.com/ciphers/mechanical-era/enigma/
