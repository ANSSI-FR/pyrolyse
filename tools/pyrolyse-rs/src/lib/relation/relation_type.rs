// use std::io::ErrorKind;

#[derive(Debug)]
pub enum RelationType {
    Pair,
    Triplet,
    Custom,
}

impl RelationType {
    // pub fn of_string(s: &String) -> Result<RelationType, ErrorKind::Other> {
    pub fn of_string(s: &str) -> Result<RelationType, String> {
        match s {
            "pair" => Ok(RelationType::Pair),
            "triplet" => Ok(RelationType::Triplet),
            "custom" => Ok(RelationType::Custom),
            _ => Err("Unexected string: {s}".to_string()),
        }
    }
}
