#[derive(Debug, Clone)]
pub enum Tower {
    WaterTower(Stats),
    FireTower(Stats)
}


impl Tower {
    pub fn new_water_tower() -> Self {
        Self::WaterTower(Stats::new(10, 2, 100))
    }

    pub fn new_fire_tower() -> Self {
        Self::FireTower(Stats::new(16, 1, 150))
    }


    /// Returns a string representation of how each tower is to be drawn in the margin
    /// 
    pub fn to_print_str(&self) -> String {
        match self {
            Tower::FireTower(stats) => Tower::fire_tower_str(stats),
            Tower::WaterTower(stats) => Tower::water_tower_str(stats)
        }
    }


    /// Returns the string representation of the fire tower
    /// 
    fn fire_tower_str(stats: &Stats) -> String {
        "".to_string()
        + "┌───┐\n"
        + "│ ^ │\n"
        + "└───┘\n"
    }


    /// Returns the string representation of the water tower
    /// 
    fn water_tower_str(stats: &Stats) -> String {
        "".to_string()
        + "┌───┐\n"
        + "│ ~ │\n"
        + "└───┘\n"
    }


    /// Creates an iterator that iterates over all types of towers
    /// 
    pub fn iter_all_towers() -> IterAllTowers {
        IterAllTowers::new()
    }
}


/// Iterates over all types of towers
/// 
pub struct IterAllTowers {
    curr: Option<Tower>
}

impl IterAllTowers {
    /// Creates a new instance of this iterator
    /// 
    fn new() -> Self {
        Self{ curr: Some(Tower::new_fire_tower()) }
    }
}


use std::iter::Iterator;
impl Iterator for IterAllTowers {
    type Item = Tower;
    fn next(&mut self) -> Option<Self::Item> {
        if self.curr.is_none() {return None}

        let res = self.curr.clone().unwrap();
        match res {
            Tower::FireTower(_) => self.curr = Some(Tower::new_water_tower()),
            Tower::WaterTower(_) => self.curr = None,
        }
        
        Some(res)
    }
}



#[derive(Debug, Clone)]
pub struct Stats {
    damage  : usize,
    speed   : usize,
    cost   : usize,
}


impl Stats {
    pub fn new(damage: usize, speed: usize, cost: usize) -> Self {
        Self { damage, speed, cost }
    }
    

    pub fn level_up(&mut self) {
        self.damage += 10;
        self.speed += 1;
    }


    /// Returns the damage of the tower
    /// 
    pub fn damage(&self) -> usize {
        self.damage
    }


    /// Returns the speed of the tower
    /// 
    pub fn speed(&self) -> usize {
        self.speed
    }


    /// Returns the cost of the tower
    /// 
    pub fn cost(&self) -> usize {
        self.cost
    }
}