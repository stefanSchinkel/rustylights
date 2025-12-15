set dotenv-load := true
set dotenv-path := ".env"

alias bp := build-pi
alias d := deploy

run:
  cargo run

test:
  cargo test

build:
  cargo build

build-pi:
  cargo build --target=armv7-unknown-linux-musleabihf

deploy:
 scp target/armv7-unknown-linux-musleabihf/debug/rustylights ${RPI_USER}@${RPI}:${RPI_PATH}/
