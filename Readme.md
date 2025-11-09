#  HIYC File Locker {-__-}

> My first Rust application! A simple but powerful file encryption/decryption tool.

##  What It Does

HIYC File Locker lets you encrypt and decrypt **any type of file** using a password. Whether it's a document, image, video, or any other file type, this tool can lock it with a password and unlock it later.

##  Features

- **Lock Files**: Encrypt any file with a password (creates a `.locked` file)
- **Unlock Files**: Decrypt locked files back to their original form
- **Show/Hide Password**: Toggle password visibility while typing
- **Password Strength Meter**: Real-time feedback on password strength (Weak/Medium/Strong)
- **Reveal in Folder**: Quickly open the file location in Finder/Explorer
- **Beautiful GUI**: Clean, user-friendly interface built with egui

## How It Works

### Locking a File
1. Click **Browse** to select a file (or type the file path)
2. Enter a strong password
3. Click **Lock File**
4. Your encrypted file is saved as `filename.ext.locked`

### Unlocking a File
1. Select the `.locked` file you want to decrypt
2. Enter the **same password** you used to lock it
3. Click **Unlock File**
4. Your original file is restored!

### Important Notes
- **Remember your password!** There's no way to recover (unless you can decrypt) it if you forget
- Use strong passwords (10+ characters with numbers and symbols)
- The original file remains unchanged when locking (you get a new `.locked` file)
- When unlocking, the `.locked` extension is automatically removed

## How to Run?

Make sure you have [Rust installed](https://rustup.rs/), then:

```bash
# Navigate to the project directory
cd hie

# Build and run the application
cargo run

# Or build for release (faster)
cargo build --release
./target/release/hie
```

## Technical Details

- **Language**: Rust
- **GUI Framework**: [egui](https://github.com/emilk/egui) with eframe
- **File Dialog**: [rfd](https://github.com/PolyMeilex/rfd) for native file picker
- **Encryption**: XOR cipher (simple encryption for learning purposes)

### Security Note
This project uses XOR encryption for educational purposes. For real-world security needs, consider using proper cryptographic libraries like:
- [age](https://github.com/str4d/rage) for file encryption
- [aes-gcm](https://docs.rs/aes-gcm/) with [argon2](https://docs.rs/argon2/) for password-based encryption

## About This Project

This is my very **first Rust application**, created to learn a new programming language, i.e. rust, and working with GUI, Encryption using cryptography, working with different OS in RUST.
## License

Feel free to use, modify, and learn from this code!

---

