#[derive(Debug)]
pub enum Tower {
    WaterTower(Stats),
    FireTower(Stats)
}

impl Tower {
    pub fn new_water_tower() -> Self {
        Self::WaterTower(Stats::new(10, 2))
    }

    pub fn new_fire_tower() -> Self {
        Self::FireTower(Stats::new(16, 1))
    }



    // fire, lighting
}

#[derive(Debug)]
pub struct Stats {
    damage  : usize,
    speed   : usize,
}

impl Stats {
    pub fn new(damage: usize, speed: usize) -> Self {
        Self { damage, speed }
    }
    
    pub fn level_up(&mut self) {
        self.damage += 10;
        self.speed += 1;
    }
}