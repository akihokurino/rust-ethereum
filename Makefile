MAKEFLAGS=--no-builtin-rules --no-builtin-variables --always-make
ROOT := $(realpath $(dir $(lastword $(MAKEFILE_LIST))))

NAME := "Rust Sample"
DESCRIPTION := "Generate token by rust"
IMAGE_FILENAME := "sample.png"
IMAGE_URL := "https://placehold.jp/3d4070/ffffff/500x500.png?text=Reveal"
AMOUNT := "10"
NETWORK := "Polygon"
ETHER := "0.01"
TO_ADDRESS := "0x0E91D6613a84d7C8b72a289D8b275AF7717C3d2E"
TOKEN_ID := "1"
MESSAGE := "world"
CONTRACT := "nft721"
CONTENT_HASH := "QmPDE4pXnFvNtqJ2889HgEQUEft8KCdyMaKKt5zzw3NuMS"

build:
	cargo build

build-impl-rust-web3:
	cargo build --lib --package impl-rust-web3

build-ipfs:
	cargo build --lib --package ipfs

build-cli:
	cargo build --bin cli

balance: build
	./target/debug/cli \
	--command balance \
	--network $(NETWORK)

send-eth: build
	./target/debug/cli \
	--command send-eth \
	--ether $(ETHER) \
	--to-address $(TO_ADDRESS) \
	--network $(NETWORK)

info: build
	./target/debug/cli \
	--command info \
	--network $(NETWORK) \
	--contract $(CONTRACT)

create-metadata: build
	./target/debug/cli \
    --command create-metadata \
  	--name $(NAME) \
    --description $(DESCRIPTION) \
    --image-filename $(IMAGE_FILENAME)

mint: build
	./target/debug/cli \
	--command mint \
	--contract $(CONTRACT) \
	--network $(NETWORK) \
	--content-hash $(CONTENT_HASH) \
	--amount $(AMOUNT)

meta-mint: build
	./target/debug/cli \
	--command mint \
	--contract meta-transaction-wallet \
	--network $(NETWORK) \
	--content-hash $(CONTENT_HASH) \
	--to-address $(TO_ADDRESS)

transfer: build
	./target/debug/cli \
	--command transfer \
	--contract $(CONTRACT) \
	--network $(NETWORK) \
	--to-address $(TO_ADDRESS) \
	--token-id $(TOKEN_ID)

deploy: build
	./target/debug/cli \
	--command deploy \
	--contract $(CONTRACT) \
	--network $(NETWORK)

extract-abi:
	cat ethereum/artifacts/contracts/Nft721.sol/Nft721.json | jq '.abi' > impl_rust_web3/src/nft_721/abi.json
	cat ethereum/artifacts/contracts/Nft721.sol/Nft721.json | jq -r '.bytecode' > impl_rust_web3/src/nft_721/bin

	cat ethereum/artifacts/contracts/Nft1155.sol/Nft1155.json | jq '.abi' > impl_rust_web3/src/nft_1155/abi.json
	cat ethereum/artifacts/contracts/Nft1155.sol/Nft1155.json | jq -r '.bytecode' > impl_rust_web3/src/nft_1155/bin