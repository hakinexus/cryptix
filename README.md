
```markdown
# 🛡️ Cryptix
**State-of-the-Art Interactive Visual Encryption Suite**

[![Rust](https://img.shields.io/badge/language-Rust-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Version](https://img.shields.io/badge/version-0.3.1-magenta.svg)]()
[![Security](https://img.shields.io/badge/encryption-XChaCha20-green.svg)]()

**Cryptix** is a high-performance, cryptographically secure tool designed to obfuscate visual data by encrypting the raw pixel matrices of images. Unlike simple filters or password-protected archives, Cryptix leverages the **XChaCha20** stream cipher to transform images into mathematically randomized noise, rendering them completely unreadable without the unique decryption key.

[Explore the Code](#-technical-architecture) • [Installation](#-installation) • [Usage](#-usage)

---

## ✨ Key Features

- **Military-Grade Encryption**: Utilizes `XChaCha20`, a modern stream cipher known for its speed and security, providing a 256-bit key and a 192-bit nonce.
- **Lossless Pixel Manipulation**: Operates directly on the RGBA byte stream. By enforcing `.png` output, Cryptix ensures that not a single bit of encrypted data is lost to compression artifacts.
- **Zero-Trace Memory**: Implements the `zeroize` trait to securely wipe sensitive keys from RAM immediately after use, preventing cold-boot attacks.
- **Sleek TUI Experience**: A polished Terminal User Interface (TUI) featuring neon aesthetics, interactive prompts, and real-time progress tracking.
- **Smart Key Management**: Generates compact, URL-safe Base64 bundles that combine the key and nonce for easy portability.

---

## ⚙️ Technical Architecture

### The Encryption Pipeline
Cryptix does not encrypt the file container; it encrypts the **visual data** itself.

1. **Decoding**: The image is loaded into a raw RGBA8 buffer.
2. **Keystream Generation**: XChaCha20 generates a pseudo-random stream of bytes based on a 32-byte key and a 24-byte nonce.
3. **XOR Splicing**: The image buffer is XORed with the keystream. Because XOR is its own inverse, applying the same keystream a second time restores the original image.
4. **Lossless Serialization**: The resulting "noise" is saved as a PNG to ensure the encrypted bytes are preserved exactly as they are.

### Security Specifications
| Component | Specification | Detail |
| :--- | :--- | :--- |
| **Cipher** | XChaCha20 | IETF variant for high-security stream encryption |
| **Key Length** | 256-bit | Generated via `OsRng` (Cryptographically Secure PRNG) |
| **Nonce** | 192-bit | Unique per image to prevent keystream reuse attacks |
| **Memory** | `Zeroize` | Guaranteed erasure of keys from memory upon `Drop` |
| **Encoding** | Base64 URL-Safe | No-pad encoding for maximum compatibility |

---

## 🚀 Installation

### Prerequisites
Ensure you have the [Rust toolchain](https://rustup.rs/) installed on your system.

### Build from Source
```bash
# Clone the repository
git clone https://github.com/yourusername/cryptix.git
cd cryptix

# Build in release mode for maximum performance
cargo build --release
```

The compiled binary will be located at `./target/release/cryptix`.

---

## 🛠️ Usage

1. **Launch the Suite**:
   ```bash
   ./target/release/cryptix
   ```

2. **Encrypting an Image**:
   - Navigate to `◈ Target Image File`.
   - Select your image from the scanned list.
   - Select `🔒 Encrypt (Lock Data)`.
   - **CRITICAL**: Copy the `SECRET KEY` block displayed in the terminal. If you lose this key, the image is mathematically unrecoverable.

3. **Decrypting an Image**:
   - Navigate to `◈ Target Image File`.
   - Select the `_locked.png` image.
   - Select `🔓 Decrypt (Unlock Data)`.
   - Paste the secret key when prompted.

---

## ⚠️ Critical Warnings

- **Lossless Formats Only**: Cryptix strictly requires `.png` for output. Using lossy formats like `.jpg` would alter the encrypted bytes, resulting in a corrupted image upon decryption.
- **Key Permanence**: There is no "Password Reset" or "Recovery" mode. The security of XChaCha20 ensures that without the key, the image is indistinguishable from random noise.

---

## 🗺️ Roadmap
- [ ] **Batch Processing**: Encrypt/Decrypt entire directories.
- [ ] **Key File Support**: Save keys to encrypted `.key` files instead of terminal output.
- [ ] **GUI Implementation**: A modern desktop wrapper using `Tauri` or `iced`.
- [ ] **Steganography**: Hide encrypted data inside seemingly normal images.

## 📄 License
Distributed under the MIT License. See `LICENSE` for more information.

---
<p align="center">
  <b>Developed with 🦀 Rust for absolute security and performance.</b>
</p>
```
