.PHONY: start-server
start-server: # CTRL+C to stop
	docker run -it --rm \
		-p 26657:26657 -p 26656:26656 -p 1337:1337 \
		-v $$(pwd):/root/code \
		--name secretdev enigmampc/secret-network-sw-dev:latest

.PHONY: snip20
snip20: # CTRL+C to stop
	bash ./testSNIP20.sh

.PHONY: factory
factory:
	bash ./testFactoryRooms.sh
	