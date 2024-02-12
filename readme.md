# QR Code CLI

A simple QR code generator for generating qr codes for urls.

## Prequsites

Note: On Linux, you may need to have xorg-dev and libxcb-composite0-dev to compile. On Debian and Ubuntu you can install them with

```Bash
sudo apt install xorg-dev libxcb-composite0-dev
```

## Installing

Clone repository

```Bash
git clone https://github.com/eliasrenman/qr-cli.git && cd qr-cli
```

Build release

```Bash
cargo build --release
```

Linking release with temporary solution of manually adding a alias to the shell for the binary output.

### Specific instructions for ZSH

```Bash
echo "# QR Code CLI Link ">> ~/.zshrc &&
echo alias qr-code=\"$(pwd)/target/release/qr-code\" >> ~/.zshrc &&
source ~/.zshrc
```

### Specific instructions for Bash

```Bash
echo "# QR Code CLI Link ">> ~/.bashrc &&
echo alias qr-code=\"$(pwd)/target/release/qr-code\" >> ~/.bashrc &&
source ~/.bashrc
```

Verify installation by running

```Bash
qr-code
```
