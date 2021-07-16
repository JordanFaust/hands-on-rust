use std::io::stdin;

#[derive(Debug)]
enum VisitorAction {
    Accept,
    AcceptWithNote { note: String },
    Refuse,
    Probation,
}

#[derive(Debug)]
struct Visitor {
    name: String,
    action: VisitorAction,
    age: i8
}

impl Visitor {
    fn new(name: &str, action: VisitorAction, age: i8) -> Self {
        Self {
            name: name.to_lowercase(),
            action,
            age,
        }
    }

    fn greet_visitor(&self) {
        match &self.action {
            VisitorAction::Accept => println!("Welcome to the treehouse, {}", self.name),
            VisitorAction::AcceptWithNote{ note } => {
                println!("Welcome to the treehouse, {}", self.name);
                println!("{}", note);
                if self.age < 21 {
                    println!("Do not serve alcohol to {}", self.name);
                }
            }
            VisitorAction::Probation => println!("{} is now a probationary member", self.name),
            VisitorAction::Refuse => println!("Do not allow {} in!", self.name)
        }
    }
}

fn what_is_your_name() -> String {
    let mut name = String::new();

    stdin()
        .read_line(&mut name)
        .expect("Failed to read line");

    name
        .trim()
        .to_lowercase()
}

fn main() {
    let mut vistor_list = vec![
        Visitor::new("bert", VisitorAction::Accept, 45),
        Visitor::new("steve", VisitorAction::AcceptWithNote{note: String::from("Lactose-free mil is in the fridge")}, 15),
        Visitor::new("fred", VisitorAction::Refuse, 30),
    ];

    loop {
        println!("Hello, what's your name?");

        let name = what_is_your_name();
        let known_visitor = vistor_list
            .iter()
            .find(|visitor| visitor.name == name);

        match known_visitor {
            Some(visitor) => visitor.greet_visitor(),
            None => {
                if name.is_empty() {
                    break;
                }
                println!("{} is not on the visitor list", name);
                vistor_list.push(Visitor::new(&name, VisitorAction::Probation, 0));
            }
        }
    }

    println!("The final list of visitors:");
    println!("{:#?}", vistor_list)
}
