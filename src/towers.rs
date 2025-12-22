use std::io::{Write, Stdout, Result as IOResult};

use crossterm::queue;
use crossterm::style::{
    Color,
    SetForegroundColor,
};

use crate::PrintLines;

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


    /// Draws the tower to the screen
    /// 
    pub fn draw(&self, stdout: &mut Stdout) -> IOResult<()> {
        match self {
            Tower::FireTower(stats) => Tower::draw_fire_tower(stdout),
            Tower::WaterTower(stats) => Tower::draw_water_tower(stdout)
        }
    }


    /// Draws fire tower
    /// 
    fn draw_fire_tower(stdout: &mut Stdout) -> IOResult<()> {
        let s = String::new()
        + "┌───┐\n"
        + "│ ^ │\n"
        + "└───┘\n";

        queue!(
            stdout,
            SetForegroundColor(Color::Red),
            PrintLines(&s),
        )?;

        stdout.flush()?;
        Ok(())
    }


    /// Returns the string representation of the water tower
    /// 
    fn draw_water_tower(stdout: &mut Stdout) -> IOResult<()> {
        let s = String::new()
        + "┌───┐\n"
        + "│ ~ │\n"
        + "└───┘\n";

        queue!(
            stdout,
            SetForegroundColor(Color::Blue),
            PrintLines(&s),
        )?;

        stdout.flush()?;
        Ok(())
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
    /// Returns a new instance of a Stats given arguments.
    /// 
    pub fn new(damage: usize, speed: usize, cost: usize) -> Self {
        Self { damage, speed, cost }
    }
    
    /// Levels up the damage and speed of stats.
    /// 
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