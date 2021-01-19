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

deployed=$(docker exec -it "$docker_name" secretcli tx compute store "/root/code/contract.wasm.gz" --from a --gas 2000000 -b block -y)
token_code_id=$(secretcli query compute list-code | jq '.[-1]."id"')
token_code_hash=$(secretcli query compute list-code | jq '.[-1]."data_hash"')
echo "Stored contract: '$token_code_id', '$token_code_hash'"

echo "Deploying contracts..."
label=$(date +"%T")

STORE_TX_HASH=$(
  secretcli tx compute instantiate $token_code_id '{"arena_name": "1", "bet_size": "1000000", "snip20_addr": '$token_contract_address', "snip20_hash": "'${token_hash:2}'"}' --from $deployer_name_a --gas 1500000 --label $label -b block -y |
  jq -r .txhash
)
wait_for_tx "$STORE_TX_HASH" "Waiting for instantiate to finish on-chain..."

contract_address1=$(docker exec -it $docker_name secretcli query compute list-contract-by-code $token_code_id | jq '.[-1].address')
echo "contract address1: '$contract_address1'"

echo "--------------------------------------------------------------------------------"
echo "register - Success Expected"
STORE_TX_HASH=$(
  secretcli tx compute execute $(echo "$contract_address1" | tr -d '"') '{"register":{"reg_addr":'$token_contract_address',"reg_hash":"'${token_hash:2}'"}}' --from a -y --gas 1500000 -b block |
  jq -r .txhash
)
wait_for_tx "$STORE_TX_HASH" "Waiting to finish on-chain..."
if [[ $(secretcli query compute tx $STORE_TX_HASH | jq -r .output_log) == *"{ \"key\": \"register_status\", \"value\": \"success\" }"* ]]; then
  echo "register failed"
  echo $(secretcli query compute tx $STORE_TX_HASH | jq -r .output_log)
  return
fi

label=$(date +"%T")

STORE_TX_HASH=$(
  secretcli tx compute instantiate $token_code_id '{"arena_name": "2", "bet_size": "2000000", "snip20_addr": '$token_contract_address', "snip20_hash": "'${token_hash:2}'"}' --from $deployer_name_a --gas 1500000 --label $label -b block -y |
  jq -r .txhash
)
wait_for_tx "$STORE_TX_HASH" "Waiting for instantiate to finish on-chain..."

contract_address2=$(docker exec -it $docker_name secretcli query compute list-contract-by-code $token_code_id | jq '.[-1].address')
echo "contract address2: '$contract_address2'"

echo "--------------------------------------------------------------------------------"
echo "register - Success Expected"
STORE_TX_HASH=$(
  secretcli tx compute execute $(echo "$contract_address2" | tr -d '"') '{"register":{"reg_addr":'$token_contract_address',"reg_hash":"'${token_hash:2}'"}}' --from a -y --gas 1500000 -b block |
  jq -r .txhash
)
wait_for_tx "$STORE_TX_HASH" "Waiting to finish on-chain..."
if [[ $(secretcli query compute tx $STORE_TX_HASH | jq -r .output_log) == *"{ \"key\": \"register_status\", \"value\": \"success\" }"* ]]; then
  echo "register failed"
  echo $(secretcli query compute tx $STORE_TX_HASH | jq -r .output_log)
  return
fi

#echo "--------------------------------------------------------------------------------"
#echo "join_arena - Fail Expected - No Room"
#STORE_TX_HASH=$(
#  secretcli tx compute execute $(echo "$token_contract_address" | tr -d '"') --label $token_label '{"send":{"recipient": '$contract_address', "amount": "2000000", "msg": "'"$(base64 -w 0 <<<'{"join_arena": {"arena_name": "bbb", "class_name": "Warrior", "secret": '$RANDOM'}}')"'"}}' --from $deployer_name_b -y --gas 1500000 -b block |
#  jq -r .txhash
#)
#wait_for_tx "$STORE_TX_HASH" "Waiting for instantiate to finish on-chain..."
#echo $(secretcli query compute tx $STORE_TX_HASH | jq -r '.output_error.generic_err.msg')

#echo "--------------------------------------------------------------------------------"
#echo "join_arena - Fail Expected - Different Bet"
#STORE_TX_HASH=$(
#  secretcli tx compute execute $(echo "$token_contract_address" | tr -d '"') --label $token_label '{"send":{"recipient": '$contract_address', "amount": "1000000", "msg": "'"$(base64 -w 0 <<<'{"join_arena": {"arena_name": "aaa", "class_name": "Warrior", "secret": '$RANDOM'}}')"'"}}' --from $deployer_name_b -y --gas 1500000 -b block |
#  jq -r .txhash
#)
#wait_for_tx "$STORE_TX_HASH" "Waiting for instantiate to finish on-chain..."
#echo $(secretcli query compute tx $STORE_TX_HASH | jq -r '.output_error.generic_err.msg')

#echo "--------------------------------------------------------------------------------"
#echo "join_arena - Fail Expected - Bad Class Name"
#STORE_TX_HASH=$(
#  secretcli tx compute execute $(echo "$token_contract_address" | tr -d '"') --label $token_label '{"send":{"recipient": '$contract_address', "amount": "2000000", "msg": "'"$(base64 -w 0 <<<'{"join_arena": {"arena_name": "aaa", "class_name": "Wasd", "secret": '$RANDOM'}}')"'"}}' --from $deployer_name_b -y --gas 1500000 -b block |
#  jq -r .txhash
#)
#wait_for_tx "$STORE_TX_HASH" "Waiting for instantiate to finish on-chain..."
#echo $(secretcli query compute tx $STORE_TX_HASH | jq -r '.output_error.generic_err.msg')

#echo "--------------------------------------------------------------------------------"
#echo "round_action - Fail Expected - Action on a arena with only one player" 
#STORE_TX_HASH=$(
#  secretcli tx compute execute --label $label '{"round_action": {"arena_name": "aaa","action_name": "Dodge"}}' --from $deployer_name_a -y --gas 1500000 -b block |
#  jq -r .txhash
#)
#wait_for_tx "$STORE_TX_HASH" "Waiting for instantiate to finish on-chain..."
#echo $(secretcli query compute tx $STORE_TX_HASH)

echo "--------------------------------------------------------------------------------"
echo "join_arena - Success Expected"
STORE_TX_HASH=$(
  secretcli tx compute execute $(echo "$token_contract_address" | tr -d '"') --label $token_label '{"send":{"recipient": '$contract_address1', "amount": "1000000", "msg": "'"$(base64 -w 0 <<<'{"join_arena": {"class_name": "Warrior", "secret": '$RANDOM'}}')"'"}}' --from $deployer_name_a -y --gas 1500000 -b block |
  jq -r .txhash
)
wait_for_tx "$STORE_TX_HASH" "Waiting for instantiate to finish on-chain..."
echo $(secretcli query compute tx $STORE_TX_HASH | jq -r .output_data_as_string)

echo "--------------------------------------------------------------------------------"
echo "join_arena - Success Expected"
STORE_TX_HASH=$(
  secretcli tx compute execute $(echo "$token_contract_address" | tr -d '"') --label $token_label '{"send":{"recipient": '$contract_address1', "amount": "1000000", "msg": "'"$(base64 -w 0 <<<'{"join_arena": {"class_name": "Warrior", "secret": '$RANDOM'}}')"'"}}' --from $deployer_name_b -y --gas 1500000 -b block |
  jq -r .txhash
)
wait_for_tx "$STORE_TX_HASH" "Waiting for instantiate to finish on-chain..."
echo $(secretcli query compute tx $STORE_TX_HASH | jq -r .output_data_as_string)

#echo "--------------------------------------------------------------------------------"
#echo "round_action - Fail Expected - Room not found" 
#STORE_TX_HASH=$(
#  secretcli tx compute execute --label $label '{"round_action": {"arena_name": "bbb","action_name": "Dodge"}}' --from $deployer_name_a -y --gas 1500000 -b block |
#  jq -r .txhash
#)
#wait_for_tx "$STORE_TX_HASH" "Waiting for instantiate to finish on-chain..."
#echo $(secretcli query compute tx $STORE_TX_HASH | jq -r '.output_error.generic_err.msg')

#echo "--------------------------------------------------------------------------------"
#echo "round_action - Fail Expected - Player not on that arena" 
#STORE_TX_HASH=$(
#  secretcli tx compute execute --label $label '{"round_action": {"arena_name": "aaa","action_name": "Dodge"}}' --from $deployer_name_c -y --gas 1500000 -b block |
#  jq -r .txhash
#)
#wait_for_tx "$STORE_TX_HASH" "Waiting for instantiate to finish on-chain..."
#echo $(secretcli query compute tx $STORE_TX_HASH | jq -r '.output_error.generic_err.msg')

#echo "--------------------------------------------------------------------------------"
#echo "round_action - Fail Expected - Bad action name" 
#STORE_TX_HASH=$(
#  secretcli tx compute execute --label $label '{"round_action": {"arena_name": "aaa","action_name": "Doe"}}' --from $deployer_name_a -y --gas 1500000 -b block |
#  jq -r .txhash
#)
#wait_for_tx "$STORE_TX_HASH" "Waiting for instantiate to finish on-chain..."
#echo $(secretcli query compute tx $STORE_TX_HASH | jq -r '.output_error.generic_err.msg')

echo "--------------------------------------------------------------------------------"
echo "round_action - Success Expected"
STORE_TX_HASH=$(
  secretcli tx compute execute $(echo "$contract_address1" | tr -d '"') '{"round_action": {"action_name": "Attack"}}' --from $deployer_name_a -y --gas 1500000 -b block |
  jq -r .txhash
)
wait_for_tx "$STORE_TX_HASH" "Waiting for instantiate to finish on-chain..."
#echo $(secretcli query compute tx $STORE_TX_HASH)

#echo "--------------------------------------------------------------------------------"
#echo "round_action - Fail Expected - Duplicating action" 
#STORE_TX_HASH=$(
#  secretcli tx compute execute --label $label '{"round_action": {"arena_name": "aaa","action_name": "Dodge"}}' --from $deployer_name_a -y --gas 1500000 -b block |
#  jq -r .txhash
#)
#wait_for_tx "$STORE_TX_HASH" "Waiting for instantiate to finish on-chain..."
#echo $(secretcli query compute tx $STORE_TX_HASH | jq -r '.output_error.generic_err.msg')

echo "--------------------------------------------------------------------------------"
echo "round_action - Success Expected"
STORE_TX_HASH=$(
  secretcli tx compute execute $(echo "$contract_address1" | tr -d '"') '{"round_action": {"action_name": "Prepare"}}' --from $deployer_name_b -y --gas 1500000 -b block |
  jq -r .txhash
)
wait_for_tx "$STORE_TX_HASH" "Waiting for instantiate to finish on-chain..."
echo $(secretcli query compute tx $STORE_TX_HASH)

echo "--------------------------------------------------------------------------------"
echo "round_action - Success Expected"
STORE_TX_HASH=$(
  secretcli tx compute execute $(echo "$contract_address1" | tr -d '"') '{"round_action": {"action_name": "Prepare"}}' --from $deployer_name_a -y --gas 1500000 -b block |
  jq -r .txhash
)
wait_for_tx "$STORE_TX_HASH" "Waiting for instantiate to finish on-chain..."
echo $(secretcli query compute tx $STORE_TX_HASH)

echo "--------------------------------------------------------------------------------"
echo "round_action - Success Expected"
STORE_TX_HASH=$(
  secretcli tx compute execute $(echo "$contract_address1" | tr -d '"') '{"round_action": {"action_name": "CounterAttack"}}' --from $deployer_name_b -y --gas 1500000 -b block |
  jq -r .txhash
)
wait_for_tx "$STORE_TX_HASH" "Waiting for instantiate to finish on-chain..."
#echo $(secretcli query compute tx $STORE_TX_HASH)

#echo "--------------------------------------------------------------------------------"
#echo "get_arena1 - Success Expected"
#secretcli q compute query $(echo "$contract_address1" | tr -d '"') '{"get_arena": {}}'
#echo "--------------------------------------------------------------------------------"
#echo "get_arena2- Success Expected"
#secretcli q compute query $(echo "$contract_address2" | tr -d '"') '{"get_arena": {}}'
echo "--------------------------------------------------------------------------------"
echo "get_general_state - Success Expected"
secretcli q compute query $(echo "$contract_address1" | tr -d '"') '{"get_general_state": {}}'
