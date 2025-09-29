use std::collections::HashMap;

#[derive(Debug)]
pub struct Entity {
    pub id: String,
    pub entity_type: String,
    pub name: String,
    pub properties: HashMap<String, String>,
}

impl Entity {
    pub fn new(id: String, entity_type: String, name: String) -> Self {
        Entity {
            id,
            entity_type,
            name,
            properties: HashMap::new(),
        }
    }

    pub fn display(&self) {
        println!("Entity: {}", self.name);
        println!("  ID: {}", self.id);
        println!("  Type: {}", self.entity_type);
        if !self.properties.is_empty() {
            println!("  Properties:");
            for (key, value) in &self.properties {
                println!("    {}: {}", key, value);
            }
        }
    }
}