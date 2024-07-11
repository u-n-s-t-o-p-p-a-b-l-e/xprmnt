use std::collections::HashMap;

fn main() {
    let mut grades = HashMap::new();

    grades.insert("Alice", 85);
    grades.insert("Bob", 78);
    grades.insert("Charlie", 90);

    for (student, grade) in &grades {
        println!("{}: {}", student, grade);
    }

    grades.insert("Alice", 88);

    for (student, grade) in &grades {
        println!("{}: {}", student, grade);
    }

    let student_name = "Charlie";
    match grades.get(student_name) {
        Some(grade) => println!("{}'s grade is {}", student_name, grade),
        None => println!("Student {} not found", student_name),
    }
}
