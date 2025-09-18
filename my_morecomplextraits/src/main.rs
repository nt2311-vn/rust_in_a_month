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

trait Magic {}
trait FightClose {}
trait FightFromDistance {}

impl FightClose for Wizard {}
impl FightClose for Ranger {}
impl FightFromDistance for Ranger {}
impl Magic for Wizard {}

fn attack_with_bow<T>(pc: &T, opponent: &mut Monster, distance: u32)
where
    T: FightFromDistance + Debug,
{
    if distance < 10 {
        opponent.health -= 10;
        println!(
            "Bow attack! Opponent's health: {}. You are now at: {pc:?}",
            opponent.health
        );
    }
}

fn attack_with_sword<T>(pc: &T, opponent: &mut Monster)
where
    T: FightClose + Debug,
{
    opponent.health -= 10;
    println!(
        "Sword attack! Opponent's health: {}. You are now at: {pc:?}",
        opponent.health
    );
}

fn fireball<T>(pc: &T, opponent: &mut Monster, distance: u32)
where
    T: Magic + Debug,
{
    if distance < 15 {
        opponent.health -= 20;
        println!(
            "A massive fireball! Opponent's health: {}. You are now at: {pc:?}",
            opponent.health
        );
    }
}

fn main() {
    let radagast = Wizard { health: 60 };
    let aragorn = Ranger { health: 80 };
    let mut uruk_hai = Monster { health: 40 };

    attack_with_sword(&radagast, &mut uruk_hai);
    attack_with_bow(&aragorn, &mut uruk_hai, 8);
    fireball(&radagast, &mut uruk_hai, 8);
}
