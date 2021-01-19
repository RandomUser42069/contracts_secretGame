# Secret Contracts Starter Pack

Simple game contract on secret network (scrt) named SecretBrawl

* Admin instanciates 5 Arenas from the main contract template (code id)
* Each Arena then has a contract with its own state, will have diferent names and different bet sizes
* Any user can join an arena and wait for an opponent. When joining the bid in BWL will be sent the contract and locked.
* A user that has entered an arena and is waiting for an opponent can leave and retrive the funds of the contract.
* When there are 2 players on the same room the game starts. Both players have the funds locked on the contract.
* Each player has to input their action in X time. After that the player forfeited and the reward can be withdrawn by the oponnent.
* When a player forfeit or has his HP = 0, it loses the match and the reward can be withdrawn by the winner.
* On the frontend force the player to only enter one room at a time!

5min limit per action on round, if eclapsed => Forfeit
3 classes to choose from
3 Actions per Round
Each action has 3 levels of power that are randomly generated on the contract
Until no more HP or 5 Rounds.

Classes: Warrior, Archer and Mage
Resouces: HP and Stamina
Actions: Attack, Block and Dodge

Classes:
  Warrior:
    - 30% more HP

  Archer:
    - 30% Dodge chance always

  Mage:
    - 30% More Damage always
  
HP: 100
Stamina: 10

Actions:
  Attack: Costs 2 Stamina + Base Damage = 20
    - Level 1: 0% Damage Increase
    - Level 2: 50% Damage Increase
    - Level 3: 100% Damage Increase
  Block: Costs 0 Stamina + Base Damage Reduction = 10
    - Level 1: 0% Damage Reduction
    - Level 2: 50% Damage Reduction
    - Level 3: 100% Damage Reduction
  Dodge: Costs 3 Stamina + Base Dodge Chance = 20%
    - Level 1: +0% Dodge chance
    - Level 2: +50% Dodge chance
    - Level 3: +100% Dodge chance

* Create Room
  Inputs: RoomName, Bet Size, Class Choosed
* Join Room
  Inputs: RoomName, Class Choosed
* Round Action
  Inputs: RoomName, ActionName
* Get Reward (Unlock when round time expires, when a player wins the match or only 1 player on the room)
  Inputs: RoomName

# Randomness
* https://build.scrt.network/dev/developing-secret-contracts.html#randomness
* https://github.com/enigmampc/SecretHoldEm.git
* Contract state with entropy field, and has max limit of len
* Player 1 when creates a room sends a random number that is added to the general entropy state
* Player 2 when joins room sends a random number that is added to the general entropy state
* To generate randomness, hash entropy state + room name + round number
# Commands
* make start-server
* make rebuild
# Links
* This contract was generated from this template: [secret-template] (https://github.com/enigmampc/secret-template)