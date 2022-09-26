use std::fmt::{self, Formatter, Display, Debug};

struct Employee {
    name: &'static str,
    id: u32,
    role: &'static str,
    salary: f32,
}

impl Display for Employee {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({}): {} - ${:>6}", self.name, self.id, self.role, self.salary)
    }
}

#[derive(Debug)]
struct Color {
    c: u8,
    m: u8,
    y: u8,
    k: u8
}

impl Display for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "0x{:x>2}{:x>2}{:x>2}{:x>2}", self.c, self.m, self.y, self.k)
    }
}

fn main() {
    for person in [
        Employee { name: "Dave", id:277, role: "Engineer", salary:10000.0},
        Employee { name: "Erica", id:1234, role: "Engineer", salary:1000.0}
    ].iter() {
        println!("{}", *person)
    }

    for color in [
        Color { c:0, m: 0, y:100, k:25 },
        Color { c:100, m:12, y:22, k:12 }
    ].iter() {
        println!("{:?}", *color);
        println!("{}", *color);
    }
}
