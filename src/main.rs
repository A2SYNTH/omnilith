mod entity;
mod relationship;

use entity::Entity;
use relationship::Relationship;

fn main() {
    println!("=== Omnilith Ontology Engine Demo ===\n");

    let mut project1 = Entity::new(
        "proj_001".to_string(),
        "Project".to_string(),
        "Art Installation 2024".to_string(),
    );
    project1.properties.insert("status".to_string(), "active".to_string());
    project1.properties.insert("budget".to_string(), "$5000".to_string());

    let mut venue1 = Entity::new(
        "venue_001".to_string(),
        "Venue".to_string(),
        "Downtown Gallery".to_string(),
    );
    venue1.properties.insert("capacity".to_string(), "150".to_string());
    venue1.properties.insert("location".to_string(), "123 Art Street".to_string());

    let contact1 = Entity::new(
        "contact_001".to_string(),
        "Contact".to_string(),
        "Alice Artist".to_string(),
    );

    let mut rel1 = Relationship::new(
        "rel_001".to_string(),
        "located_at".to_string(),
        "proj_001".to_string(),
        "venue_001".to_string(),
    );
    rel1.properties.insert("start_date".to_string(), "2024-03-01".to_string());

    let rel2 = Relationship::new(
        "rel_002".to_string(),
        "participates_in".to_string(),
        "contact_001".to_string(),
        "proj_001".to_string(),
    );

    println!("--- Entities ---");
    project1.display();
    println!();
    venue1.display();
    println!();
    contact1.display();
    println!();

    println!("--- Relationships ---");
    rel1.display();
    println!();
    rel2.display();
}
