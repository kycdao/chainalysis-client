use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Category {
    Sanctions,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AddressSanctions {
    pub identifications: Vec<Identification>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Identification {
    pub category: Category,
    pub name: Option<String>,
    pub description: Option<String>,
    pub url: Option<String>,
}
