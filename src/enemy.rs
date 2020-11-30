
pub struct Enemy {
    pub hp: i32
}

impl Enemy {
    pub fn new(hp: i32) -> Self {
        Enemy { hp }
    }

    pub fn generic_enemy() -> Self {
        Enemy::new(80)
    }

    fn take_damage(&mut self, damage: i32) {
        self.hp -= damage;
    }
}