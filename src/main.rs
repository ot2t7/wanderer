struct Something {
    pub name: String
}

impl Something {
    pub fn useMe(&self) {
        println!("do something");
    }
}

fn main() {
    let newSomething = Something {
        name: String::from("lol")
    };

    Something::useMe(&newSomething);
}
