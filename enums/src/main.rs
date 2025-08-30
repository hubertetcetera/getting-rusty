fn main() {
    enum CardinalDirection {
        North,
        East,
        South,
        West,
    }

    let north = CardinalDirection::North;
    let east = CardinalDirection::East;
    let south = CardinalDirection::South;
    let west = CardinalDirection::West;

    fn move_to(direction: CardinalDirection) {
        match direction {
            CardinalDirection::North => println!("Move North"),
            CardinalDirection::East => println!("Move East"),
            CardinalDirection::South => println!("Move South"),
            CardinalDirection::West => println!("Move West"),
        }
    }

    move_to(north);
    move_to(east);
    move_to(south);
    move_to(west);

    enum Animal {
        Dog(String),
        Cat { name: String, age: u8 },
        Bird,
    }

    let dog = Animal::Dog("Tucker".to_string());

    let cat = Animal::Cat {
        name: String::from("Mittens"),
        age: 3,
    };

    let bird = Animal::Bird;

    impl Animal {
        fn name(&self) -> String {
            match self {
                Animal::Dog(name) => name.clone(),
                Animal::Cat { name, .. } => name.clone(),
                Animal::Bird => String::from("bird"),
            }
        }

        fn age(&self) -> u8 {
            match self {
                Animal::Cat { age, .. } => *age,
                _ => 0,
            }
        }
    }

    println!("Dog's name is {}", dog.name());
    println!(
        "Cat's name is {} and it is {} years old",
        cat.name(),
        cat.age()
    );
    println!("Bird's name is {}", bird.name());
}
