use std::fmt::Debug;

struct Monster {
    health: i32,
}

#[derive(Debug)]
struct Wizard {
    health: i32,
}

#[derive(Debug)]
struct Ranger {
    health: i32,
}

trait DisplayHealth {
    fn health(&self) -> i32;
}

trait FightClose: Debug {
    fn attack_with_sword(&self, opponent: &mut Monster) {
        opponent.health -= 10;
        println!(
            "Sword attack! Opponent's health: {}. You are now at: {:?}",
            opponent.health, self
        );
    }

    fn attack_with_hand(&self, opponent: &mut Monster) {
        opponent.health -= 2;
        println!(
            "Hand attack! Opponent's health: {}. You are now at: {:?}",
            opponent.health, self
        );
    }
}

impl FightClose for Wizard {}
impl FightClose for Ranger {}

trait FightFromDistance: Debug {
    fn attack_with_bow(&self, opponent: &mut Monster, distance: u32) {
        if distance < 10 {
            opponent.health -= 10;
            println!(
                "Bow attack! Opponent's health: {}. You are now at: {:?}",
                opponent.health, self
            );
        }
    }

    fn attack_with_rock(&self, opponent: &mut Monster, distance: u32) {
        if distance < 3 {
            opponent.health -= 4;
        }

        println!(
            "Rock attack! Opponent's health: {}. You are now at: {:?}",
            opponent.health, self
        );
    }
}

impl FightFromDistance for Ranger {}
fn main() {
    let radagast = Wizard { health: 60 };
    let aragorn = Ranger { health: 80 };
    let mut uruk_hai = Monster { health: 40 };

    radagast.attack_with_sword(&mut uruk_hai);
    aragorn.attack_with_bow(&mut uruk_hai, 8);
}
