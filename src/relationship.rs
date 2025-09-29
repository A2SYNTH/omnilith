use std::collections::HashMap;

#[derive(Debug)]
pub struct Relationship {
    pub id: String,
    pub relationship_type: String,
    pub from_entity_id: String,
    pub to_entity_id: String,
    pub properties: HashMap<String, String>,
}

impl Relationship {
    pub fn new(
        id: String,
        relationship_type: String,
        from_entity_id: String,
        to_entity_id: String,
    ) -> Self {
        Relationship {
            id,
            relationship_type,
            from_entity_id,
            to_entity_id,
            properties: HashMap::new(),
        }
    }

    pub fn display(&self) {
        println!("Relationship: {}", self.relationship_type);
        println!("  ID: {}", self.id);
        println!("  From: {}", self.from_entity_id);
        println!("  To: {}", self.to_entity_id);
        if !self.properties.is_empty() {
            println!("  Properties:");
            for (key, value) in &self.properties {
                println!("    {}: {}", key, value);
            }
        }
    }
}