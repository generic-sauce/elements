# Elements
Elements is a competitive 1 vs 1 arena game inspired by the TV show "Avatar The Last Airbender"

## Play an early version right now
You can try this game online on [**our website**](https://generic-sauce.de)

Bugs are likely. Chrome browser is recommended. Have fun :D

## Installation of Native Client and Game Server
```bash
apt install libssl-dev libudev-dev pkg-config

git clone https://github.com/generic-sauce/elements.git

# native client compilation
cd native_client
cargo run --release menu

# game server compilation
cd game_server
cargo run --release --no-default-features --features=game-server
```

## Push with git lfs
```bash
git add <files>
git commit
git push
git config -f .lfsconfig lfs.url https://github.com/generic-sauce/elements.git/info/lfs
git push sauce
rm .lfsconfig
```

