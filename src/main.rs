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

pub fn instruct_turtle(turtle: &mut Turtle, instructions: String, dist: f64, rot: f64) {
    let (mut lX, mut lY, mut sX, mut sY) = (0.0, 0.0, 0.0, 0.0);
    
    for i in instructions.chars() {
        match i {
            'F' => turtle.forward(dist),
            'f' => {turtle.pen_up(); turtle.forward(dist); turtle.pen_down()},
            '+' => turtle.left(rot),
            '-' => turtle.right(rot),
            '|' => turtle.right(180.0),
            '{' => turtle.begin_fill(),
            '}' => turtle.end_fill(),
            _ => {}
        }

        let pos = turtle.position();
        if pos.x > lX {
            lX = pos.x;
        } else if pos.x < sX {
            sX = pos.x;
        }
        if pos.y > lY {
            lY = pos.y;
        } else if pos.y < sY {
            sY = pos.y;
        }
        turtle.drawing_mut().set_center(((0.0 - lX - sX) / 2.0, (0.0 - sY - lY) / 2.0))
    }
}

fn main() {
    let mut turtle = Turtle::new();
    turtle.drawing_mut().set_title("L-System Turtle!");
    turtle.set_speed("instant");

    let axiom = "A";
    let steps = 5;
    let rules = |x| match x {
        'A' => "-BF+AFA+FB-",
        'B' => "+AF-BFB-FA+",
        'F' => "F",
        'f' => "f",
        '+' => "+",
        '-' => "-",
        '|' => "|",
    //  '[' => "",
    //  ']' => "",
        '{' => "{",
        '}' => "}",
        _ => ""
    };

    instruct_turtle(&mut turtle, l_system(axiom, steps, &rules), 4.0, 90.0);
}
