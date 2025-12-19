pub enum Tower {
    BasicMonkey(Stats)
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