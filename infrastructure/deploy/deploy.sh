#!/bin/bash


ELEMENTS_ROOT_DIR="${PWD%/*/*}"
MASTER_SERVER_PATH="$PWD/master_server"
GAME_SERVER_PATH="$PWD/game_server"


############## CLEANUP OLD CONTAINER ##############
cd "$MASTER_SERVER_PATH"
echo "stopping old master container..."
docker-compose down
echo "stopping old master container: Done"

cd "$GAME_SERVER_PATH"
echo "stopping old game container..."
docker-compose down
echo "stopping old game container: Done"


############## REPOSITORY ##############
# pulling
cd "$ELEMENTS_ROOT_DIR"
echo "Pulling Repository..."
git pull
echo "Pulling Repository: Done"

# building
echo "Building Game Server + Master Server..."
/home/sauce/.cargo/bin/cargo build --release --package=master_server --package=game_server
echo "Building Game Server + Master Server: Done"


############## MASTER SERVER ##############
cd "$MASTER_SERVER_PATH"

rm -rf "$MASTER_SERVER_PATH/target"
mkdir -p "$MASTER_SERVER_PATH/target/release"

cp -r "$ELEMENTS_ROOT_DIR/target/release/master_server" "$MASTER_SERVER_PATH/target/release/elements-master-server"

echo "Starting new master-server..."
docker-compose up -d
echo "Starting new master-server: Done"


############## GAME SERVER ##############
# deploy
mkdir -p "$GAME_SERVER_PATH"
cd "$GAME_SERVER_PATH"

rm -rf "$GAME_SERVER_PATH/target"
rm -rf "$GAME_SERVER_PATH/res"

mkdir -p "$GAME_SERVER_PATH/target/release"

cp -r "$ELEMENTS_ROOT_DIR/target/release/game_server" "$GAME_SERVER_PATH/target/release/elements-game-server"
cp -r "$ELEMENTS_ROOT_DIR/res" "$GAME_SERVER_PATH/res"

echo "Starting new game-servers..."
docker-compose up -d
echo "Starting new game-servers: Done"

# web server
if [ ! "$1" == "--skip-web" ]; then
	cd "$ELEMENTS_ROOT_DIR/infrastructure/rocket-server"

	./do-it.sh
fi

