# Elements
Elements is a competitive 1 vs 1 arena game inspired by the TV show "Avatar The Last Airbender"

## Play and early version
You can try this game online on [**our website**](https://generic-sauce.de)

Chrome browser is recommended
Have fun :D

## Installation of Native Client and Game Server
```bash
apt install libssl-dev libudev-dev pkg-config

git clone https://github.com/generic-sauce/elements.git

# native client compilation
cargo run --release menu

# game server compilation
cargo run --release --no-default-features --features=game-server
```
