_list:
    just --list

openssl-command:
    nix run nixpkgs#openssl -- pkcs8 --inform der --in pkcs8-version.der --outform pem --out -


# Use `openssl` to generate a PKS8 DER private key - it should be given the password "password"
generate-keys:
    nix run nixpkgs#openssl -- genpkey -algorithm ed25519 -outform der -out private_key.der
    nix run nixpkgs#openssl -- pkcs8 --inform der --in private_key.der -topk8 -outform der -out pkcs8-version.der
    rm private_key.der


# Use the decrypt_key_inlined example binary to decrypt the PKCS8 DER private key, assuming that the password is "password"
generate-and-decrypt-keys:
    just generate-keys
    cargo run --example=decrypt_key_inlined
