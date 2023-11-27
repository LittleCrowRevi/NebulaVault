use bevy::prelude::*;
use crate::entities::components::*;

#[derive(Bundle)]
pub struct BaseRace {
    pub vital: VitalStats,
    pub core: CoreStats
}

#[derive(Component)]
pub struct RaceName(pub String);

#[derive(Component)]
pub struct Human; 

#[derive(Bundle)]
pub struct HumanBundle {
    pub base_stats: BaseRace,
    pub name: RaceName,
    pub tag: Human,
}

impl Default for HumanBundle { 
    fn default() -> Self {
        Self { 
            name: RaceName(String::from("Human")),
            base_stats: BaseRace {
                core: CoreStats {
                    strength: 10, 
                    agility: 10, 
                    constitution: 10, 
                    fortune: 10,
                    wisdom: 10,
                    intelligence: 10
                },
                vital: VitalStats {
                    health: 100,
                    energy: 100,
                    mana: 100
                }
            },
            tag: Human
        }
    }
}

#[derive(Bundle)]
pub struct HalfAngelBundle {
    pub base_stats: BaseRace,
    pub name: RaceName,
    pub tag: HalfAngel
}

#[derive(Component)]
pub struct HalfAngel;

impl Default for HalfAngelBundle {
    fn default() -> Self {
        Self {
            name: RaceName(String::from("Half-Angel")),
            base_stats: BaseRace {
                core: CoreStats {
                    strength: 6,
                    agility: 10,
                    constitution: 8,
                    fortune: 12,
                    wisdom: 14,
                    intelligence: 10
                },
                vital: VitalStats {
                    health: 10,
                    energy: 10,
                    mana: 15
                }
            },
            tag: HalfAngel
        }
    }
}

#[derive(Bundle)]
pub struct FaerieBundle {
    pub base_stats: BaseRace,
    pub name: RaceName,
    pub tag: Faerie
}

#[derive(Component)]
pub struct Faerie;

impl Default for FaerieBundle {
    fn default() -> Self {
        Self {
            name: RaceName(String::from("Faerie")),
            base_stats: BaseRace {
                core: CoreStats {
                    strength: 6,
                    agility: 15,
                    constitution: 6,
                    fortune: 12,
                    wisdom: 10,
                    intelligence: 12
                },
                vital: VitalStats {
                    health: 10,
                    energy: 5,
                    mana: 20
                }
            },
            tag: Faerie
        }
    }
}

