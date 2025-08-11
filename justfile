set dotenv-load := true
set dotenv-path := ".env"


build:
	cargo build

build-rpi:
	cargo build --target=armv7-unknown-linux-musleabihf

deploy:
	scp target/armv7-unknown-linux-gnueabihf/debug/rustylights ${RPI_USER}@${RPI}:${RPI_PATH}/
