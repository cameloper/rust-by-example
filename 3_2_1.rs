#![allow(dead_code)]

enum Stage {
    Beginner,
    Advanced,
}

enum Role {
    Student,
    Teacher,
}

fn main() {
    use Stage::{Beginner, Advanced};
    use Role::*;

    let stage = Beginner;

    let role = Student;

    match stage {
        Beginner => println!("Beginner are starting their learning journey"),
        Advanced => println!("Advanced learners are mastering their subjects")
    }

    match role {
        Student => println!("Students are acquiring knowledge!"),
        Teacher => println!("Teachers are spreading knowledge!"),
    }
}
