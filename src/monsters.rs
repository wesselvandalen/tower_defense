#[derive(Debug, Clone)]
pub enum Monster {
    Zombie(MonsterStats),
}

impl Monster {
    /// Creates a new zombie instance of monster.
    /// 
    pub fn new_zombie() -> Self {
        Self::Zombie(MonsterStats::new(10, 1, 50))
    }

    /// Handles the damage taken by a tower attack.
    ///
    pub fn take_damage(&mut self, damage: usize) {
        if damage >= self.health() {
            self.set_health(0);
        } else {
            self.set_health(self.health() - damage);
        }
    }

    pub fn set_health(&mut self, new_health: usize) {
        match self {
            Self::Zombie(stats) => stats.set_health(new_health)
        }
    }

    pub fn health(&self) -> usize {
        match self {
            Self::Zombie(stats) => stats.health()
        }
    }

    pub fn speed(&self) -> usize {
        match self {
            Self::Zombie(stats) => stats.speed()
        }
    }

    pub fn damage(&self) -> usize {
        match self {
            Self::Zombie(stats) => stats.damage()
        }
    }
}

#[derive(Debug, Clone)]
pub struct MonsterStats {
    damage: usize,
    speed: usize,
    health: usize,
}

impl MonsterStats {
    /// Returns a new instance of a MonsterStats given arguments.
    /// 
    pub fn new(damage: usize, speed: usize, health: usize) -> Self {
        Self {
            damage,
            speed,
            health,
        }
    }

    /// Sets the health to a certain value (health).
    /// 
    pub fn set_health(&mut self, health: usize) {
        self.health = health;
    }

    /// Returns the damage of a monster.
    /// 
    pub fn damage(&self) -> usize {
        self.damage
    }

    /// Returns the speed of a monster.
    ///
    pub fn speed(&self) -> usize {
        self.speed
    }

    /// Returns the health of a monster.
    /// 
    pub fn health(&self) -> usize {
        self.health
    }
}
