
pub struct Enemy {
    pub hp: i32,
    pub name: String
}

impl Enemy {
    pub fn new(hp: i32, name: String) -> Self {
        Enemy { hp, name }
    }

    pub fn generic_enemy() -> Self {
        Enemy::new(80, String::from("generic enemy"))
    }

    pub fn take_damage(&mut self, damage: i32) {
        self.hp -= damage;
    }
}