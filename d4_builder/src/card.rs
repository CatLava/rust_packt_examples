use std::collections::BTreeMap;


// Card game similar to Magic

#[derive(Debug, PartialEq, PartialOrd, Ord, Eq)]
pub enum Trigger {
    BattleCry,
    Death,
    EnemyDeath,
    Damage,
}


// Cards have numerous properties about them
#[derive(Debug, PartialEq)]
pub struct Card {
    pub name: String,
    pub strength: i32,
    pub health: i32,
    pub cost: i32,
    pub abilities: Vec<Ability>,
    pub triggers: BTreeMap<Trigger, String>,
}

#[derive(Debug, PartialEq)]
pub struct CardBuilder {
    pub name: String,
    pub strength: Option<i32>,
    pub health: Option<i32>,
    pub cost: Option<i32>,
    pub abilities: Vec<Ability>,
    pub triggers: BTreeMap<Trigger, String>,
}

impl CardBuilder {
    pub fn new(name: String) -> Self{
        CardBuilder{
            name,
            strength:None,
            health:None,
            cost:None,
            abilities:Vec::new(),
            triggers: BTreeMap::new(),
        }
    }

    pub fn strength(mut self, s:i32) -> Self{
        self.strength = Some(s);
        self
    }

    pub fn trigger(mut self, t: Trigger, s:String) -> Self {
        self.triggers.insert(t,s);
        self
    }

    pub fn build(self)->Card{
        Card{
            name: self.name,
            strength: self.strength.unwrap_or(1),
            health: self.health.unwrap_or(1),
            cost: self.cost.unwrap_or(1),
            abilities: self.abilities,
            triggers: self.triggers,
        }
    }
}
