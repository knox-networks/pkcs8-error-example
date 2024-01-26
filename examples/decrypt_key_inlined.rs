use pkcs_8_breaking_example::Keypair;

fn main() {
    let path = "./pkcs8-version.der";
    let password: &[u8] = b"password";
    let doc = std::fs::read(path).unwrap();
    let info = pkcs8::EncryptedPrivateKeyInfo::try_from(doc.as_ref()).unwrap();
    let encrypted = info;
    let password = password;
    let secret = encrypted.decrypt(password).unwrap();
    let pk_info: pkcs8::PrivateKeyInfo = secret.decode_msg().unwrap();
    let secret_key_bytes = pk_info.private_key;
    assert_eq!(secret_key_bytes.len(), 34);

    //Strip 0x04, 0x20 header
    assert_eq!(secret_key_bytes[0..2], [0x04, 0x20]);
    let stripped_bytes = &secret_key_bytes[2..];
    let sk = ed25519_zebra::SigningKey::try_from(stripped_bytes)
        .map_err(|e| e.to_string())
        .unwrap();
    let keypair = Keypair::new(sk);

    println!("Public Key: {:x?}", keypair.vk.as_ref());
}
