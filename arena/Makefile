all:
	RUSTFLAGS='-C link-arg=-s' cargo build --release --target wasm32-unknown-unknown
	cp ./target/wasm32-unknown-unknown/release/*.wasm ./contract.wasm
	## The following line is not necessary, may work only on linux (extra size optimization)
	# wasm-opt -Os ./contract.wasm -o ./contract.wasm
	cat ./contract.wasm | gzip -9 > ./contract.wasm.gz

clean:
	cargo clean
	-rm -f ./contract.wasm ./contract.wasm.gz

.PHONY: start-server
start-server: # CTRL+C to stop
	docker run -it --rm \
		-p 26657:26657 -p 26656:26656 -p 1337:1337 \
		-v $$(pwd):/root/code \
		--name secretdev enigmampc/secret-network-sw-dev:latest

.PHONY: create-snip20
create-snip20: # CTRL+C to stop
	bash ./snip20/init-contract.sh


.PHONY: rebuild
rebuild:
	#sudo cargo clean
	rm -f ./contract.wasm ./contract.wasm.gz
	cargo wasm
	cargo schema
	docker run --rm -v $(shell pwd):/contract \
	--mount type=volume,source=$(shell basename $(shell pwd))_cache,target=/code/target \
	--mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
	enigmampc/secret-contract-optimizer 
	bash ./tests/test.sh
