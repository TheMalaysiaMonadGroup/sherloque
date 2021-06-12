use crate::generate::{generate, Language};

pub fn schema1() -> String {
    "
CREATE TABLE person (
    id int not null,
    name varchar(255),
    PRIMARY KEY (id)
);

CREATE TABLE pet (
    id int NOT NULL,
    kind int NOT NULL,
    owner_id int NOT NULL,
    PRIMARY KEY (id),
    FOREIGN KEY (owner_id) REFERENCES person(id)
);

    "
    .to_string()
}

#[test]
pub fn tests() {
    let test_cases = vec![
        ("SELECT name FROM person",
            "() -> [{name: String}]"),

        ("SELECT name FROM person WHERE id = @@id",
            "{id: String} -> [{name: String}]"),

        ("SELECT * FROM person",
            "() -> [{id: String, name: String}]"),

        ("SELECT person.name, COUNT(*) FROM person INNER JOIN pet ON person.id = pet.owner_id GROUP BY person.id",
            "() -> [{'person.name': String, 'count(*)': String}]"),
    ];
    test_cases
        .into_iter()
        .enumerate()
        .for_each(|(index, (operation, expected_type))| {
            let result = generate(
                schema1(),
                index.to_string(),
                operation.to_string(),
                Language::TypesOnly,
            );
            assert_eq!(result, Ok(expected_type.to_string()))
        });
}
