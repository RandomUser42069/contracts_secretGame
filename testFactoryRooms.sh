#!/bin/bash

cd factory
#cargo clean
rm -f ./contract.wasm ./contract.wasm.gz
cargo wasm
cargo schema
docker run --rm -v $PWD:/contract \
--mount type=volume,source=factory_cache,target=/code/target \
--mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
enigmampc/secret-contract-optimizer

cd ../arena
#cargo clean
rm -f ./contract.wasm ./contract.wasm.gz
cargo wasm
cargo schema
docker run --rm -v $PWD:/contract \
--mount type=volume,source=arena_cache,target=/code/target \
--mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
enigmampc/secret-contract-optimizer

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

token_label=secretBrawler
deployer_name_a=a
deployer_name_b=b
deployer_name_c=c

deployer_address_a=$(secretcli keys show -a $deployer_name_a)
echo "Deployer address a: '$deployer_address_a'"

deployer_address_b=$(secretcli keys show -a $deployer_name_b)
echo "Deployer address b: '$deployer_address_b'"

deployer_address_c=$(secretcli keys show -a $deployer_name_c)
echo "Deployer address c: '$deployer_address_c'"

token_contract_address=$(docker exec -it $docker_name secretcli query compute list-contract-by-code 1 | jq '.[-1].address')
token_hash="$(secretcli query compute contract-hash $(echo "$token_contract_address" | tr -d '"'))"
echo "token contract address: '$token_contract_address'"

export STORE_TX_HASH=$(
  docker exec -it $docker_name secretcli tx compute execute --label $token_label '{"create_viewing_key": {"entropy":"MyPassword123"}}' --from $deployer_name_a -y --gas 3000000 -b block |
  jq -r .txhash
)
wait_for_tx "$STORE_TX_HASH" "Waiting for instantiate to finish on-chain..."
viewing_key_a=$(docker exec $docker_name secretcli query compute tx $STORE_TX_HASH | jq '.output_data_as_string | fromjson.create_viewing_key.key')

export STORE_TX_HASH=$(
  docker exec -it $docker_name secretcli tx compute execute --label $token_label '{"create_viewing_key": {"entropy":"MyPassword123"}}' --from $deployer_name_b -y --gas 3000000 -b block |
  jq -r .txhash
)
wait_for_tx "$STORE_TX_HASH" "Waiting for instantiate to finish on-chain..."
viewing_key_b=$(docker exec $docker_name secretcli query compute tx $STORE_TX_HASH | jq '.output_data_as_string | fromjson.create_viewing_key.key')

echo "--------------------------------------------------------------------------------"
echo "Success Expected - A Balance"
echo $(docker exec -it $docker_name secretcli query compute query $(echo "$token_contract_address" | tr -d '"')  '{"balance":{"address":"'$deployer_address_a'","key":'$viewing_key_a'}}' )

echo "--------------------------------------------------------------------------------"
echo "Success Expected - B Balance"
echo $(docker exec -it $docker_name secretcli query compute query $(echo "$token_contract_address" | tr -d '"')  '{"balance":{"address":"'$deployer_address_b'","key":'$viewing_key_b'}}' )

deployed=$(docker exec -it "$docker_name" secretcli tx compute store "/root/code/factory/contract.wasm.gz" --from a --gas 2000000 -b block -y)
token_code_id=$(secretcli query compute list-code | jq '.[-1]."id"')
token_code_hash=$(secretcli query compute list-code | jq '.[-1]."data_hash"')
echo "Stored contract: '$token_code_id', '$token_code_hash'"

deployed=$(docker exec -it "$docker_name" secretcli tx compute store "/root/code/arena/contract.wasm.gz" --from a --gas 2000000 -b block -y)
token_code_id_arena=$(secretcli query compute list-code | jq '.[-1]."id"')
token_code_hash_arena=$(secretcli query compute list-code | jq '.[-1]."data_hash"')
echo "Stored contract: '$token_code_id_arena', '$token_code_hash_arena'"

echo "Deploying factory..."
label=$(date +"%T")

STORE_TX_HASH=$( 
  secretcli tx compute instantiate $token_code_id '{"entropy": "'$RANDOM'", "arena_contracts_code_id": 1, "arena_contracts_code_hash": "'12131'", "snip20_contract_code_address": '$token_contract_address', "snip20_contract_code_hash": "'${token_hash:2}'"}' --from $deployer_name_a --gas 1500000 --label $label -b block -y |
  jq -r .txhash
)
wait_for_tx "$STORE_TX_HASH" "Waiting for instantiate to finish on-chain..."

factory_contract_address=$(docker exec -it $docker_name secretcli query compute list-contract-by-code $token_code_id | jq '.[-1].address')
echo "factory_contract_address: '$factory_contract_address'"

echo "--------------------------------------------------------------------------------"
echo "user a viewkey factory - Success Expected"
STORE_TX_HASH=$(
  secretcli tx compute execute $(echo "$factory_contract_address" | tr -d '"') '{"create_viewing_key": {"entropy": "'$RANDOM'"}}' --from $deployer_name_a -y --gas 1500000 -b block |
  jq -r .txhash
)
wait_for_tx "$STORE_TX_HASH" "Waiting for instantiate to finish on-chain..."
user_factory_vk_a=$(docker exec $docker_name secretcli query compute tx $STORE_TX_HASH | jq '.output_data_as_string | fromjson.viewing_key.key')

echo "--------------------------------------------------------------------------------"
echo "user a viewkey factory check - Success Expected"
secretcli q compute query $(echo "$factory_contract_address" | tr -d '"') '{"is_key_valid": {"address": "'$deployer_address_a'", "viewing_key":'$user_factory_vk_a'}}'

echo "--------------------------------------------------------------------------------"
echo "user b viewkey factory - Success Expected"
STORE_TX_HASH=$(
  secretcli tx compute execute $(echo "$factory_contract_address" | tr -d '"') '{"create_viewing_key": {"entropy": "'$RANDOM'"}}' --from $deployer_name_b -y --gas 1500000 -b block |
  jq -r .txhash
)
wait_for_tx "$STORE_TX_HASH" "Waiting for instantiate to finish on-chain..."
user_factory_vk_b=$(docker exec $docker_name secretcli query compute tx $STORE_TX_HASH | jq '.output_data_as_string | fromjson.viewing_key.key')

echo "--------------------------------------------------------------------------------"
echo "user b viewkey factory check - Success Expected"
secretcli q compute query $(echo "$factory_contract_address" | tr -d '"') '{"is_key_valid": {"address": "'$deployer_address_b'", "viewing_key":'$user_factory_vk_b'}}'

echo "--------------------------------------------------------------------------------"
echo "admin code id change - Success Expected"
STORE_TX_HASH=$(
  secretcli tx compute execute $(echo "$factory_contract_address" | tr -d '"') '{"change_arena_contract_code_id": {"code_id": '$token_code_id_arena', "code_hash":'$token_code_hash_arena'}}' --from $deployer_name_a -y --gas 1500000 -b block |
  jq -r .txhash
)
wait_for_tx "$STORE_TX_HASH" "Waiting for instantiate to finish on-chain..."
echo $(docker exec $docker_name secretcli query compute tx $STORE_TX_HASH)

echo "--------------------------------------------------------------------------------"
echo "admin code id change check - Success Expected"
secretcli q compute query $(echo "$factory_contract_address" | tr -d '"') '{"arena_contract_code_id": {}}'

echo "--------------------------------------------------------------------------------"
echo "instanciate room - Success Expected"
STORE_TX_HASH=$(
  secretcli tx compute execute $(echo "$factory_contract_address" | tr -d '"') '{"new_arena_instanciate": {"name": "'$RANDOM'", "entropy":'$RANDOM'}}' --from $deployer_name_a -y --gas 1500000 -b block |
  jq -r .txhash
)
wait_for_tx "$STORE_TX_HASH" "Waiting for instantiate to finish on-chain..."
echo $(docker exec $docker_name secretcli query compute tx $STORE_TX_HASH)

sleep 10
echo "--------------------------------------------------------------------------------"
echo "arenas query factory - Success Expected"
first_arena_contract=$(docker exec $docker_name secretcli q compute query $(echo "$factory_contract_address" | tr -d '"') '{"arenas": {}}' | jq -r .arenas.arenas[0])
echo $first_arena_contract

echo "--------------------------------------------------------------------------------"
echo "join_arena - Success Expected"
STORE_TX_HASH=$(
  secretcli tx compute execute $(echo "$token_contract_address" | tr -d '"') --label $token_label '{"send":{"recipient": "'$first_arena_contract'", "amount": "1000000", "msg": "'"$(base64 -w 0 <<<'{"join_arena": {"class_name": "Warrior" ,"secret": '$RANDOM'}}')"'"}}' --from $deployer_name_a -y --gas 1500000 -b block |
  jq -r .txhash
)
wait_for_tx "$STORE_TX_HASH" "Waiting for instantiate to finish on-chain..."
echo $(docker exec $docker_name secretcli query compute tx $STORE_TX_HASH)

echo "--------------------------------------------------------------------------------"
echo "join_arena - Success Expected"
STORE_TX_HASH=$(
  secretcli tx compute execute $(echo "$token_contract_address" | tr -d '"') --label $token_label '{"send":{"recipient": "'$first_arena_contract'", "amount": "1000000", "msg": "'"$(base64 -w 0 <<<'{"join_arena": {"class_name": "Warrior" ,"secret": '$RANDOM'}}')"'"}}' --from $deployer_name_b -y --gas 1500000 -b block |
  jq -r .txhash
)
wait_for_tx "$STORE_TX_HASH" "Waiting for instantiate to finish on-chain..."
echo $(docker exec $docker_name secretcli query compute tx $STORE_TX_HASH)

echo "--------------------------------------------------------------------------------"
echo "get arena state - Success Expected"
secretcli q compute query $(echo "$first_arena_contract" | tr -d '"') '{"get_state": {"user_address": "'$deployer_address_a'", "user_viewkey": '$user_factory_vk_a'}}'

echo "--------------------------------------------------------------------------------"
echo "get arena state - Success Expected"
secretcli q compute query $(echo "$first_arena_contract" | tr -d '"') '{"get_state": {"user_address": "'$deployer_address_a'", "user_viewkey": '$user_factory_vk_b'}}'

