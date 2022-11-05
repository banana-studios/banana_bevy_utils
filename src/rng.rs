use bevy_turborand::*;

pub trait RandomNumbers {
    /// Rolls dice, using the classic 3d6 type of format: n is the number of dice, die_type is the size of the dice.
    fn roll_dice(&mut self, n: i32, dice_type: i32) -> i32;
}

impl RandomNumbers for GlobalRng {
    fn roll_dice(&mut self, n: i32, die_type: i32) -> i32 {
        (0..n).map(|_| self.i32(1..die_type + 1)).sum()
    }
}
