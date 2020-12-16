# Elements
Nice game with elements :3


## Usage
You can try this game online on [our website](https://generic-sauce.de). Have fun :D

## Installation of Native Client and Game Server
```bash
apt install libssl-dev libudev-dev pkg-config

git clone https://github.com/generic-sauce/elements.git

# native client compilation
cargo run --release menu

# game server compilation
cargo run --release --no-default-features --features=game-server
```
