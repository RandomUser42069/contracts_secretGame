#!/bin/bash

docker_name=secretdev

function secretcli() {
  docker exec "$docker_name" secretcli "$@";
}

function wait_for_tx() {
  until (secretcli q tx "$1"); do
      sleep 5
  done
}

export SGX_MODE=SW

deployer_name_a=a
deployer_name_b=b

deployer_address_a=$(secretcli keys show -a $deployer_name_a)
echo "Deployer address a: '$deployer_address_a'"

deployer_address_b=$(secretcli keys show -a $deployer_name_b)
echo "Deployer address b: '$deployer_address_b'"

docker exec -it "$docker_name" secretcli tx compute store "/root/code/snip20/contract.wasm.gz" --from a --gas 2000000 -b block -y
token_code_id=$(secretcli query compute list-code | jq '.[-1]."id"')
token_code_hash=$(secretcli query compute list-code | jq '.[-1]."data_hash"')
echo "Stored contract: '$token_code_id', '$token_code_hash'"

echo "Deploying contract..."
label=secretBrawler

export STORE_TX_HASH=$(
  secretcli tx compute instantiate $token_code_id '{"name": "secretBrawler", "symbol": "BWL", "decimals": 6, "initial_balances": [{"address": "'$deployer_address_a'", "amount": "100000000"}], "prng_seed": "cenas123", "config":{"public_total_supply":true}}' --from $deployer_name_a --gas 1500000 --label $label -b block -y |
  jq -r .txhash
)
wait_for_tx "$STORE_TX_HASH" "Waiting for instantiate to finish on-chain..."

## Send to b
echo $(docker exec -it $docker_name secretcli tx compute execute --label $label '{"send":{"recipient":"'$deployer_address_b'","amount":"50000000"}}' --from $deployer_name_a -y --gas 3000000 -b block)

contract_address=$(docker exec -it $docker_name secretcli query compute list-contract-by-code $token_code_id | jq -r '.[-1].address')
echo "contract address: '$contract_address'"

