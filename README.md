# Elements
Elements is a competitive 1 vs 1 arena game inspired by the TV show "Avatar The Last Airbender"

## Play an early version right now
You can try this game online on [**our website**](https://generic-sauce.de)

Bugs are likely. Chrome browser is recommended. Have fun :D

## Installation of Native Client and Game Server
```bash
apt install libssl-dev libudev-dev pkg-config

git clone https://github.com/generic-sauce/elements.git

# run native client
cargo run --release --package native_client

# run game server compilation
cargo run --release --package game_server -- -d localhost
```
