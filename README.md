# Factory
- Manage Admin Commands
    - Change Arena CodeID
    - Instanciate Arena (arena_name, bet_size)
        - Needs to send the FactoryViewingKey to the arena
        - Needs to register for SNIP20 the new arena
        - Needs to save this arena address on state
- Manage Viewing Keys (Factory and Users)
    - On Factory instanciate, create a FVK that is saved to the state
    - Users create their viewing key here to interact with the game
# Arena
- Manage the state of an arena
- Query, that can be authenticated. To get general state of the game, incluing the rooms that the auth user is in!