pub enum Tower {
    BasicMonkey(Stats),
    FireTower(Stats)
}

impl Tower {
    pub fn new_fire_tower() -> Tower {
        Tower::FireTower((Stats { damage: 16, speed: 1 }))
    }



    // fire, lighting
}

pub struct Stats {
    damage  : usize,
    speed   : usize,
}

impl Stats {
    pub fn level_up(&mut self) {
        self.damage += 10;
        self.speed += 1;
    }
}