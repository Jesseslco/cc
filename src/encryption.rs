type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub struct EncryptionChipher {
    pub chip_name: &'static str,
    pub encrypt: fn(&mut u8) -> Result<()>,
    pub decrypt: fn(&mut u8) -> Result<()>,
}

pub const ROT_13_ENCRYPTION_CHIPHER: &'static EncryptionChipher = &EncryptionChipher {
    chip_name: "rot-13",
    encrypt: encrypt_byte_rot13,
    decrypt: decrypt_byte_rot_13,
};

pub fn encrypt_byte_rot13(c: &mut u8) -> Result<()> {
    if *c >= b'a' && *c <= b'z' {
        *c = *c + 13;
        if *c > b'z' {
            *c = *c - 26;
        }
    } else if *c >= b'A' && *c <= b'Z' {
        *c = *c + 13;
        if *c > b'Z' {
            *c = *c - 26;
        }
    }
    Ok(())
}

pub fn decrypt_byte_rot_13(c: &mut u8) -> Result<()> {
    if *c >= b'a' && *c <= b'z' {
        *c -= 13;
        if *c < b'a' {
            *c += 26;
        }
    } else if *c >= b'A' && *c <= b'Z' {
        *c -= 13;
        if *c < b'A' {
            *c = *c + 26;
        }
    } else {
        // do nothing
    }
    Ok(())
}

#[cfg(test)]
mod tests {}
