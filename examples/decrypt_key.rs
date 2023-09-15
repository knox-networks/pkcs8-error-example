use rust_crypto_example::*;

fn main() {
    let keypair = Keypair::decrypt_from_file("./pk.der", b"password").unwrap();
    println!("Public Key: {:x?}", keypair.vk.as_ref());
}