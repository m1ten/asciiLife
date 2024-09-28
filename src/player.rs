struct Physique {
    height: f32,
    weight: f32,
}

struct Skill {
    speed: u8,
    strength: u8,
    agility: u8,
    stamina: u8,
}

enum CrimeType {
    Burglary,
    Theft,
    Assault,
    Murder,
}

struct Crime {
    name: CrimeType,
    severity: u8,
    description: String,
    year: u16
}

pub(crate) struct Player {
    first_name: String,
    last_name: String,
    age: u8,
    physique: Physique,
    skill: Skill,
    karma: f32,
    crimes: Option<Vec<Crime>>
}

impl Player {
    /// Creates a new [`Player`].
    pub fn new(first_name: &str, last_name: &str, age: u8, height: f32, weight: f32, speed: u8, strength: u8, agility: u8, stamina: u8) -> Player {
        Player {
            first_name: first_name.to_string(),
            last_name: last_name.to_string(),
            age,
            physique: Physique { height, weight },
            skill: Skill { speed, strength, agility, stamina },
            karma: 0.0,
            crimes: None
        }
    }

    /// Returns the full name of this [`Player`].
    pub fn get_full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    /// Returns the bmi of this [`Player`].
    pub fn get_bmi(&self) -> f32 {
        self.physique.weight / (self.physique.height * self.physique.height)
    }

    
}