# Rust-Ciphers

# Project Structure 
```
crypto_project
│
├── src
│   ├── main.rs               # Main module
│   ├── input.rs              # Input handling module
│   ├── rsa.rs                # RSA functionality module
│   ├── aes.rs                # AES functionality module
│   ├── ecies.rs              # ECIES functionality module
│   └── ecdsa.rs              # ECDSA functionality module
│
└── lib
    ├── enums_and_structs     # Folder for enums and structs
    │   ├── enums.rs          # Enums module
    │   └── structs.rs        # Structs module
    ├── key_generation        # Folder for key generation functionality
    │   ├── rsa_keygen.rs     # RSA key generation module
    │   ├── aes_keygen.rs     # AES key generation module
    │   └── ecdsa_keygen.rs   # ECDSA key generation module
    ├── encryption            # Folder for encryption functionality
    │   ├── rsa_encrypt.rs    # RSA encryption module
    │   ├── aes_encrypt.rs    # AES encryption module
    │   └── ecies_encrypt.rs  # ECIES encryption module
    ├── decryption            # Folder for decryption functionality
    │   ├── rsa_decrypt.rs    # RSA decryption module
    │   ├── aes_decrypt.rs    # AES decryption module
    │   └── ecies_decrypt.rs  # ECIES decryption module
    ├── signing               # Folder for signing functionality
    │   ├── rsa_sign.rs       # RSA signing module
    │   └── ecdsa_sign.rs     # ECDSA signing module
    └── verification          # Folder for verification functionality
        ├── rsa_verify.rs     # RSA verification module
        └── ecdsa_verify.rs   # ECDSA verification module
```