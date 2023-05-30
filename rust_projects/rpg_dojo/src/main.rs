use rand::Rng;
use std::io;

enum Class {
    Wizard,
    Warrior,
    Rogue,
}

impl Class {
    fn to_string(&self) -> String {
        match self {
            Class::Wizard => "Wizard".to_owned(),
            Class::Warrior => "Warrior".to_owned(),
            Class::Rogue => "Rogue".to_owned(),
        }
    }
}

struct Entity {
    name: String,
    class: Class,
    health: i32,
    defense: bool,
    stunned: bool,
    dodging: bool,
    poisoned: i32,
}

fn generate_enemy() -> Entity {
    let mut rng = rand::thread_rng();
    let class = match rng.gen_range(1..=3) {
        1 => Class::Wizard,
        2 => Class::Warrior,
        3 => Class::Rogue,
        _ => panic!("Invalid class!"),
    };
    let health = match class {
        Class::Wizard => rng.gen_range(50..=75),
        Class::Warrior => rng.gen_range(75..=100),
        Class::Rogue => rng.gen_range(50..=100),
    };
    Entity {
        name: "Enemy".to_owned(),
        class,
        health,
        defense: false,
        stunned: false,
        dodging: false,
        poisoned: 0,
    }
}

fn wizard_action(entity: &mut Entity, target: &mut Entity, action: &str) {
    let mut rng = rand::thread_rng();
    match action {
        "attack" => {
            if target.defense {
                target.defense = false;
                println!(
                    "{} miss the spell, {} was defending itself!",
                    entity.name, target.name
                );
            } else if entity.stunned {
                entity.stunned = false;
                println!("{} is stunned and cannot attack!", entity.name);
            } else {
                let spell_damage = rng.gen_range(10..20);
                println!("{} casts a spell for {} damage!", entity.name, spell_damage);
                target.health -= spell_damage;
            }
        }
        "defend" => {
            entity.defense = true;
            println!("{} casts a defensive spell!", entity.name);
        }
        "secondary" => {
            if target.defense {
                target.defense = false;
                println!(
                    "{} teleported way too far, {} was defending itself!",
                    entity.name, target.name
                );
            } else if entity.stunned {
                entity.stunned = false;
                println!("{} is stunned and cannot attack!", entity.name);
            } else {
                let teleport_damage = rng.gen_range(5..15);
                let success = rng.gen_bool(0.5);
                if success {
                    println!(
                        "{} teleports behind {} and deals {} damage!",
                        entity.name, target.name, teleport_damage
                    );
                    target.health -= teleport_damage;
                } else {
                    println!("{} failed to teleport!", entity.name);
                }
            }
        }
        _ => panic!("Invalid action!"),
    }
    if entity.poisoned > 0 {
        entity.health -= entity.poisoned;
        println!(
            "{} is poisoned and lose {} health!",
            entity.name, entity.poisoned
        );
    }
}

fn warrior_action(entity: &mut Entity, target: &mut Entity, action: &str) {
    let mut rng = rand::thread_rng();
    match action {
        "attack" => {
            if target.defense {
                target.defense = false;
                println!(
                    "{} fails the attack, {} was defending itself!",
                    entity.name, target.name
                );
            } else if entity.stunned {
                entity.stunned = false;
                println!("{} is stunned and cannot attack!", entity.name);
            } else {
                let heavy_damage = rng.gen_range(15..25);
                println!(
                    "{} attacks with a heavy blow for {} damage!",
                    entity.name, heavy_damage
                );
                target.health -= heavy_damage;
            }
        }
        "defend" => {
            entity.defense = true;
            println!("{} raises their shield!", entity.name);
        }
        "secondary" => {
            if target.defense {
                target.defense = false;
                println!(
                    "{} hits the air, {} was defending itself!",
                    entity.name, target.name
                );
            } else if entity.stunned {
                entity.stunned = false;
                println!("{} is stunned and cannot attack!", entity.name);
            } else {
                let stun_chance = rng.gen_bool(0.3);
                if stun_chance {
                    println!(
                        "{} delivers a stunning blow, stunning {}!",
                        entity.name, target.name
                    );
                    target.stunned = true;
                } else {
                    println!("{} swings wildly, but misses!", entity.name);
                }
            }
        }
        _ => panic!("Invalid action!"),
    }
    if entity.poisoned > 0 {
        entity.health -= entity.poisoned;
        println!(
            "{} is poisoned and lose {} health!",
            entity.name, entity.poisoned
        );
    }
}

fn rogue_action(entity: &mut Entity, target: &mut Entity, action: &str) {
    let mut rng = rand::thread_rng();
    match action {
        "attack" => {
            if target.defense {
                target.defense = false;
                println!(
                    "{} fails the attack, {} was defending itself!",
                    entity.name, target.name
                );
            } else if entity.stunned {
                entity.stunned = false;
                println!("{} is stunned and cannot attack!", entity.name);
            } else {
                let sneak_damage = rng.gen_range(5..15);
                println!(
                    "{} sneaks behind {} and deals {} damage!",
                    entity.name, target.name, sneak_damage
                );
                target.health -= sneak_damage;
            }
        }
        "defend" => {
            let dodge_chance = rng.gen_bool(0.5);
            if dodge_chance {
                entity.dodging = true;
                println!(
                    "{} readies themselves for an incoming attack, increasing their dodge chance!",
                    entity.name
                );
            } else {
                println!("{} tries to dodge but fails!", entity.name);
            }
        }
        "secondary" => {
            if target.defense {
                target.defense = false;
                println!(
                    "{} fails to poison, {} was defending itself!",
                    entity.name, target.name
                );
            } else if entity.stunned {
                entity.stunned = false;
                println!("{} is stunned and cannot attack!", entity.name);
            } else {
                let poison_damage = rng.gen_range(5..10);
                let success = rng.gen_bool(0.7);
                if success {
                    target.poisoned += poison_damage;
                    println!(
                        "{} poisons {} for {} damage over time!",
                        entity.name, target.name, poison_damage
                    );
                } else {
                    println!(
                        "{} tries to poison {}, but fails!",
                        entity.name, target.name
                    );
                }
            }
        }
        _ => panic!("Invalid action!"),
    }
    if entity.poisoned > 0 {
        entity.health -= entity.poisoned;
        println!(
            "{} is poisoned and lose {} health!",
            entity.name, entity.poisoned
        );
    }
}

fn main() {
    // Player chooses class
    println!("Choose your class:");
    println!("1. Wizard");
    println!("2. Warrior");
    println!("3. Rogue");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let class = match input.trim().parse::<i32>() {
        Ok(1) => Class::Wizard,
        Ok(2) => Class::Warrior,
        Ok(3) => Class::Rogue,
        _ => panic!("Invalid class!"),
    };
    let mut player = Entity {
        name: "Player".to_owned(),
        class,
        health: 100,
        defense: false,
        stunned: false,
        dodging: false,
        poisoned: 0,
    };
    let mut enemy = generate_enemy();
    let mut rng = rand::thread_rng();

    println!("You are a {}!", player.class.to_string());
    println!("You are fighting a {}!", enemy.class.to_string());
    loop {
        println!(
            "\n{}: Health: {}, Defense: {}",
            player.name, player.health, player.defense
        );
        println!("{}: Health: {}", enemy.name, enemy.health);
        println!("What action will you perform?");
        println!("1. Attack");
        println!("2. Defend");
        println!("3. Secondary Attack");

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let choice = input.trim().parse::<u32>().unwrap();
        let action;

        match choice {
            1 => {
                action = "attack".to_owned();
            }
            2 => {
                action = "defend".to_owned();
            }
            3 => {
                action = "secondary".to_owned();
            }
            _ => {
                println!("Invalid choice!");
                continue;
            }
        }

        match player.class {
            Class::Wizard => wizard_action(&mut player, &mut enemy, &action),
            Class::Warrior => warrior_action(&mut player, &mut enemy, &action),
            Class::Rogue => rogue_action(&mut player, &mut enemy, &action),
        }

        let enemy_choice = rng.gen_range(1..=3);
        match enemy_choice {
            1 => {
                let action = "attack".to_owned();
                match enemy.class {
                    Class::Wizard => wizard_action(&mut enemy, &mut player, &action),
                    Class::Warrior => warrior_action(&mut enemy, &mut player, &action),
                    Class::Rogue => rogue_action(&mut enemy, &mut player, &action),
                }
            }
            2 => {
                let action = "defend".to_owned();
                match enemy.class {
                    Class::Wizard => wizard_action(&mut enemy, &mut player, &action),
                    Class::Warrior => warrior_action(&mut enemy, &mut player, &action),
                    Class::Rogue => rogue_action(&mut enemy, &mut player, &action),
                }
            }
            3 => {
                let action = "secondary".to_owned();
                match enemy.class {
                    Class::Wizard => wizard_action(&mut enemy, &mut player, &action),
                    Class::Warrior => warrior_action(&mut enemy, &mut player, &action),
                    Class::Rogue => rogue_action(&mut enemy, &mut player, &action),
                }
            }
            _ => panic!("Invalid choice!"),
        }

        if enemy.health <= 0 {
            println!("{} has been defeated!", enemy.name);
            break;
        }

        if player.health <= 0 {
            println!("{} has been defeated!", player.name);
            break;
        }
    }
}
