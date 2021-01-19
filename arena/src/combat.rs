// Agressive > Economy > Defensive > Agressive
// Attack => Agressive
// Defend => Defensive
// Prepare => Economy

// Randomness => On attack moves multipliers of a dice

use crate::state::{Action, Class};

pub fn round_result(
        player1_hp: i32,
        player1_preparation: bool,
        player2_hp: i32, 
        player2_preparation: bool,
        player1_action: &Action, 
        player2_action: &Action, 
        player1_class: &Class, 
        player2_class: &Class
    ) -> (i32, bool, i32, bool) {
    let mut new_player1_hp = player1_hp;
    let mut new_player1_preparation = player1_preparation;
    let mut new_player2_hp = player2_hp;
    let mut new_player2_preparation = player2_preparation;
    
    match (player1_action.name.as_str(), player2_action.name.as_str()){
        ("Attack", "Attack") => {
            new_player1_hp = new_player1_hp - player2_class.base_attack;
            new_player2_hp = new_player2_hp - player1_class.base_attack;
            new_player1_preparation = false;
            new_player2_preparation = false;
        },
        ("Attack", "Block") => {
            new_player1_hp = new_player1_hp;
            new_player2_hp = new_player2_hp;
            new_player1_preparation = false;
            new_player2_preparation = false;
        },
        ("Attack", "Prepare") => {
            new_player1_hp = new_player1_hp;
            new_player2_hp = new_player2_hp - player1_class.base_attack;
            new_player1_preparation = false;
            new_player2_preparation = true;
        },
        ("Attack", "UnblockableAttack") => {
            new_player1_hp = new_player1_hp - player2_class.base_attack;
            new_player2_hp = new_player2_hp - player1_class.base_attack;
            new_player1_preparation = false;
            new_player2_preparation = false;
        },
        ("Attack", "CounterAttack") => {
            new_player1_hp = new_player1_hp - player2_class.base_attack;
            new_player2_hp = new_player2_hp;
            new_player1_preparation = false;
            new_player2_preparation = false;
        },
        ("Block", "Attack") => {
            new_player1_hp = new_player1_hp;
            new_player2_hp = new_player2_hp;
            new_player1_preparation = false;
            new_player2_preparation = false;
        },
        ("Block", "Block") => {
            new_player1_hp = new_player1_hp;
            new_player2_hp = new_player2_hp;
            new_player1_preparation = false;
            new_player2_preparation = false;
        },
        ("Block", "Prepare") => {
            new_player1_hp = new_player1_hp;
            new_player2_hp = new_player2_hp;
            new_player1_preparation = false;
            new_player2_preparation = true;
        },
        ("Block", "UnblockableAttack") => {
            new_player1_hp = new_player1_hp - player2_class.base_attack;
            new_player2_hp = new_player2_hp;
            new_player1_preparation = false;
            new_player2_preparation = false;
        },
        ("Block", "CounterAttack") => {
            new_player1_hp = new_player1_hp;
            new_player2_hp = new_player2_hp;
            new_player1_preparation = false;
            new_player2_preparation = false;
        },
        ("Prepare", "Attack") => {
            new_player1_hp = new_player1_hp - player2_class.base_attack;
            new_player2_hp = new_player2_hp;
            new_player1_preparation = true;
            new_player2_preparation = false;
        },
        ("Prepare", "Block") => {
            new_player1_hp = new_player1_hp;
            new_player2_hp = new_player2_hp;
            new_player1_preparation = true;
            new_player2_preparation = false;
        },
        ("Prepare", "Prepare") => {
            new_player1_hp = new_player1_hp;
            new_player2_hp = new_player2_hp;
            new_player1_preparation = true;
            new_player2_preparation = true;
        },
        ("Prepare", "UnblockableAttack") => {
            new_player1_hp = new_player1_hp - player2_class.base_attack;
            new_player2_hp = new_player2_hp;
            new_player1_preparation = true;
            new_player2_preparation = false;
        },
        ("Prepare", "CounterAttack") => {
            new_player1_hp = new_player1_hp;
            new_player2_hp = new_player2_hp;
            new_player1_preparation = true;
            new_player2_preparation = false;
        },
        ("UnblockableAttack", "Attack") => {
            new_player1_hp = new_player1_hp - player2_class.base_attack;
            new_player2_hp = new_player2_hp - player1_class.base_attack;
            new_player1_preparation = false;
            new_player2_preparation = false;
        },
        ("UnblockableAttack", "Block") => {
            new_player1_hp = new_player1_hp;
            new_player2_hp = new_player2_hp - player1_class.base_attack;
            new_player1_preparation = false;
            new_player2_preparation = false;
        },
        ("UnblockableAttack", "Prepare") => {
            new_player1_hp = new_player1_hp;
            new_player2_hp = new_player2_hp - player1_class.base_attack;
            new_player1_preparation = false;
            new_player2_preparation = true;
        },
        ("UnblockableAttack", "UnblockableAttack") => {
            new_player1_hp = new_player1_hp - player2_class.base_attack;
            new_player2_hp = new_player2_hp - player1_class.base_attack;
            new_player1_preparation = false;
            new_player2_preparation = false;
        },
        ("UnblockableAttack", "CounterAttack") => {
            new_player1_hp = new_player1_hp - player2_class.base_attack;
            new_player2_hp = new_player2_hp;
            new_player1_preparation = false;
            new_player2_preparation = false;
        },
        ("CounterAttack", "Attack") => {
            new_player1_hp = new_player1_hp;
            new_player2_hp = new_player2_hp - player1_class.base_attack;
            new_player1_preparation = false;
            new_player2_preparation = false;
        },
        ("CounterAttack", "Block") => {
            new_player1_hp = new_player1_hp;
            new_player2_hp = new_player2_hp;
            new_player1_preparation = false;
            new_player2_preparation = false;
        },
        ("CounterAttack", "Prepare") => {
            new_player1_hp = new_player1_hp;
            new_player2_hp = new_player2_hp;
            new_player1_preparation = false;
            new_player2_preparation = true;
        },
        ("CounterAttack", "UnblockableAttack") => {
            new_player1_hp = new_player1_hp;
            new_player2_hp = new_player2_hp - player1_class.base_attack;
            new_player1_preparation = false;
            new_player2_preparation = false;
        },
        ("CounterAttack", "CounterAttack") => {
            new_player1_hp = new_player1_hp;
            new_player2_hp = new_player2_hp;
            new_player1_preparation = false;
            new_player2_preparation = false;
        },
        _ => println!("Ain't special"),
    }
    
    return (new_player1_hp, new_player1_preparation, new_player2_hp, new_player2_preparation)
}













