#![allow(dead_code)]
#![allow(clippy::upper_case_acronyms)]

fn main() {
    // enums variant can hold any type and amount of associate data

    #[derive(Debug, Clone)]
    struct PlayerData {
        name: String,
        health: i32,
    }

    impl PlayerData {
        fn is_weak(&self) -> bool {
            self.health < 30
        }

        fn is_strong(&self) -> bool {
            self.health > 70
        }

        fn is_dead(&self) -> bool {
            self.health == 0
        }
    }

    #[derive(Debug)]
    enum Player {
        Regular(PlayerData),
        Advance(PlayerData),
        NPC(PlayerData),
    }

    impl Player {
        fn intro(&self) {
            let name = match &self {
                Player::Regular(_) => "Regular Player",
                Player::Advance(_) => "Advance Player",
                Player::NPC(_) => "NPC Player",
            };
            println!("Hello! I am a {}", name);
        }
    }

    let player_data = PlayerData {
        name: String::from("Brian"),
        health: 100,
    };

    let is_old = player_data.is_weak();
    let is_dead = player_data.is_dead();
    let is_strong = player_data.is_strong();
    println!("The player is weak = {}", is_old);
    println!("The player is strong = {}", is_strong);
    println!("The player is dead = {}", is_dead);

    let regular_player = Player::Regular(player_data.clone());
    let advance_player = Player::Advance(player_data.clone());
    let npc = Player::NPC(player_data.clone());

    println!("Advance Player = {:?}", advance_player);
    println!("Regular Player = {:?}", regular_player);
    println!("NPC = {:?}", npc);

    regular_player.intro();
    advance_player.intro();
    npc.intro();
}
