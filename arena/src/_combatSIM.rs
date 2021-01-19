pub struct State {
    classes: Vec<Class>
}

pub struct Class {
    pub name: String,
    pub base_hp: i32,
    pub base_attack: i32,
    pub base_dodge_chance: i32,
    pub actions: Vec<Action>
}
pub struct Action {
    pub name: String,
    pub preparation_needed: bool
}

// Agressive > Economy > Defensive > Agressive
// Attack => Agressive
// Defend => Defensive
// Prepare => Economy

// Randomness => On attack moves multipliers of a dice
//
fn main() {
    let state = State {
        classes: vec![
        Class {
            name: "Warrior".to_string(),
            base_hp: 125,
            base_attack: 20,
            base_dodge_chance: 0,
            actions: vec![
             Action {
                name: "Attack".to_string(),
                preparation_needed: false
            },
             Action {
                name: "UnblockableAttack".to_string(),
                preparation_needed: true,
            },
             Action {
                name: "Block".to_string(),
                preparation_needed: false,
            },
             Action {
                name: "CounterAttack".to_string(),
                preparation_needed: true,
            },
             Action {
                name: "Prepare".to_string(),
                preparation_needed: false
            }]
        },
        Class {
            name: "Archer".to_string(),
            base_hp: 100,
            base_attack: 25,
            base_dodge_chance: 20,
            actions: vec![
                 Action {
                    name: "Attack".to_string(),
                    preparation_needed: false,
                },
                 Action {
                    name: "DoubleAttack".to_string(),
                    preparation_needed: true,
                },
                 Action {
                    name: "Block".to_string(),
                    preparation_needed: false,
                },
                 Action {
                    name: "IncreaseDodgeChance".to_string(),
                    preparation_needed: true,
                },
                 Action {
                    name: "Prepare".to_string(),
                    preparation_needed: false
                }]
            }
        ]
    };
    let player1_class = state.classes.iter().find(|class| class.name == "Warrior".to_string()).unwrap();
    let player2_class =  state.classes.iter().find(|class| class.name == "Warrior".to_string()).unwrap();
    
    let player1_action = player1_class.actions.iter().find(|action| action.name == "Attack".to_string()).unwrap();
    let player2_action = player2_class.actions.iter().find(|action| action.name == "Prepare".to_string()).unwrap();
    //let player1_hp = player1_class.base_hp;
    let player1_hp = 105;
    //let player2_hp = player2_class.base_hp;
    let player2_hp = 105;
    let player1_preparation = false;
    let player2_preparation = true;
    
    let (player1_hp, player1_preparation, player2_hp, player2_preparation) = round_result(
        player1_hp, 
        player1_preparation,
        player2_hp, 
        player2_preparation,
        player1_action, 
        player2_action, 
        player1_class, 
        player2_class
    );

    println!("Player1 HP: {}, Preparation: {}", player1_hp, player1_preparation);
    println!("Player2 HP: {}, Preparation: {}", player2_hp, player2_preparation);
}

fn round_result(
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













