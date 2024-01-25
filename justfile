
openssl-command:
    nix run nixpkgs#openssl -- pkcs8 --inform der --in pk.der --outform pem --out -
