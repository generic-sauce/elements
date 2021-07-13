#!/bin/bash


set -e
ELEMENTS_ROOT_DIR="${PWD%/*/*}"
MASTER_SERVER_PATH="$PWD/master_server"
GAME_SERVER_PATH="$PWD/game_server"
ELEMENTS_BUILD_CONTAINER_NAME="elements-build-container"


############## CLEANUP OLD CONTAINER ##############
cd "$MASTER_SERVER_PATH"
echo "stopping old master container..."
docker-compose down --timeout 1
echo "stopping old master container: Done"

cd "$GAME_SERVER_PATH"
echo "stopping old game container..."
docker-compose down --timeout 1
echo "stopping old game container: Done"


############## REPOSITORY ##############
# pulling
cd "$ELEMENTS_ROOT_DIR"
echo "Pulling Repository..."
git pull
echo "Pulling Repository: Done"

# start build container, if not already running
if [ ! "$( docker container inspect -f '{{.State.Status}}' "$ELEMENTS_BUILD_CONTAINER_NAME" )" == "running" ]; then
	echo "starting build container"
	docker run -v "$ELEMENTS_ROOT_DIR":"/root/elements" --rm -d --name "$ELEMENTS_BUILD_CONTAINER_NAME" bruno1996/elements2-server sleep infinity
fi

# build
echo "Building Game Server + Master Server..."
docker exec -w /root/elements $ELEMENTS_BUILD_CONTAINER_NAME /root/.cargo/bin/rustup update
docker exec -w /root/elements $ELEMENTS_BUILD_CONTAINER_NAME /root/.cargo/bin/cargo build --release --package master_server --package game_server
echo "Building Game Server + Master Server: Done"


############## MASTER SERVER ##############
cd "$MASTER_SERVER_PATH"

if [ -d "$MASTER_SERVER_PATH/target" ]; then
	rm -rf "$MASTER_SERVER_PATH/target"
fi
mkdir -p "$MASTER_SERVER_PATH/target/release"

cp "$ELEMENTS_ROOT_DIR/target/release/master_server" "$MASTER_SERVER_PATH/target/release/elements-master-server"

echo "Starting new master-server..."
docker-compose up -d
echo "Starting new master-server: Done"


############## GAME SERVER ##############
# deploy
mkdir -p "$GAME_SERVER_PATH"
cd "$GAME_SERVER_PATH"

if [ -d "$GAME_SERVER_PATH/target" ]; then
	rm -rf "$GAME_SERVER_PATH/target"
fi
if [ -d "$GAME_SERVER_PATH/res" ]; then
	rm -rf "$GAME_SERVER_PATH/res"
fi

mkdir -p "$GAME_SERVER_PATH/target/release"

cp "$ELEMENTS_ROOT_DIR/target/release/game_server" "$GAME_SERVER_PATH/target/release/elements-game-server"
cp -r "$ELEMENTS_ROOT_DIR/res" "$GAME_SERVER_PATH/res"

echo "Starting new game-servers..."
docker-compose up -d
echo "Starting new game-servers: Done"

# web server
if [ ! "$1" == "--skip-web" ]; then
	cd "$ELEMENTS_ROOT_DIR/infrastructure/rocket-server"

	./do-it.sh
fi
