list:
    just --list

openssl-command:
    nix run nixpkgs#openssl -- pkcs8 --inform der --in pk.der --outform pem --out -


generate-keys:
    nix run nixpkgs#openssl -- genpkey -algorithm ed25519 -outform der -out private_key.der
    nix run nixpkgs#openssl -- pkcs8 --inform der --in private_key.der -topk8 -outform der -out pkcs8-version.der
    rm private_key.der
