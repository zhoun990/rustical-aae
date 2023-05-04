use super::*;

#[derive(Debug, Serialize, Deserialize, Default, Clone, Type)]
pub struct Relation {
    pub id: i32,
    pub name: String,
    pub impression: i32,
    pub relation_type: RelationType,
    pub last_met_timestamp: u32,
}
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Type)]
pub enum RelationType {
    Child,
    Parent,
    Sibling,
    Partner,
    Acquaintance,
    Clan,
}
#[derive(PartialEq, Debug, Serialize, Deserialize, Clone, Type)]
pub enum Gender {
    Male,
    Female,
}

impl Default for Gender {
    fn default() -> Self {
        Gender::Male
    }
}
impl Default for RelationType {
    fn default() -> Self {
        RelationType::Acquaintance
    }
}
impl Gender {
    pub fn random() -> Self {
        if percentage(1, 2) {
            Self::Male
        } else {
            Self::Female
        }
    }
    pub fn from_string(from: String) -> Self {
        if from == "Male" {
            Self::Male
        } else {
            Self::Female
        }
    }
}
impl RelationType {
    pub fn from_string(from: String) -> Self {
        if from == "Child" {
            Self::Child
        } else if from == "Parent" {
            Self::Parent
        } else if from == "Sibling" {
            Self::Sibling
        } else if from == "Partner" {
            Self::Partner
        } else if from == "Acquaintance" {
            Self::Acquaintance
        } else {
            Self::Clan
        }
    }
    pub fn to_string(&self) -> String {
        match self {
            RelationType::Child => "Child",
            RelationType::Parent => "Parent",
            RelationType::Sibling => "Sibling",
            RelationType::Partner => "Partner",
            RelationType::Acquaintance => "Acquaintance",
            RelationType::Clan => "Clan",
        }
        .to_string()
    }
}
