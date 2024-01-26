fn main() {
    let path = "./pkcs8-version.der";
    let password: &[u8] = b"password";
    let doc = std::fs::read(path).unwrap();
    let info = pkcs8::EncryptedPrivateKeyInfo::try_from(doc.as_ref()).unwrap();
    let encrypted = info;
    let password = password;
    let secret = encrypted.decrypt(password).unwrap();
    let pk_info: pkcs8::PrivateKeyInfo = secret.decode_msg().unwrap();
    let len = pk_info.private_key.len();
    println!("Secret Key len: {len}, key: {:?}", pk_info.private_key);
    /*
    let sk = ed25519_zebra::SigningKey::try_from(pk_info.private_key)
        .map_err(|e| e.to_string())
        .unwrap();
    let keypair = Keypair::new(sk);

    println!("Public Key: {:x?}", keypair.vk.as_ref());
    */
}
