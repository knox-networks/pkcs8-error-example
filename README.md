
Reproduce the error by running `just generate-and-decrypt-keys`. This does two things:

1. Use `openssl` to generate an ed25519 secret key, and then convert it into a PKCS8-formatted
DER key. `openssl` prompts for a password, which should be "password".

2. Use the `decrypt_key_inlined` example rust binary to attempt to decrypt the key, assuming the
password "password"
