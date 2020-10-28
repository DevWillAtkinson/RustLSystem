#![allow(non_snake_case)]
extern crate turtle;
use turtle::Turtle;

pub fn l_system<'a>(axiom: &str, steps: u32, rules: &dyn Fn(char) -> &'a str) -> String {
    let mut input = String::from(axiom);
    for _ in 0..steps {
        let out = input.chars().map(rules).collect();
        input = out;
    }

    input.to_string()
}

fn main() {
    let mut turtle = Turtle::new();
    turtle.drawing_mut().set_size([600, 600]);
    turtle.left(90.0);

    let axiom = "A";
    let steps = 6;
    let rules = |x| match x {
        'A' => "-BF+AFA+FB-",
        'B' => "+AF-BFB-FA+",
        '+' => "+",
        '-' => "-",
        'F' => "F",
        _ => ""
    };
    
    let out = l_system(axiom, steps, &rules);

    println!("{}", out);

    let distance = 8.0;
    let angle = 90.0;
    for i in out.chars() {
        match i {
            'F' => turtle.forward(distance),
            '+' => turtle.right(angle),
            '-' => turtle.left(angle),
            _ => {}
        }
    }
}
