#![allow(non_snake_case)]
extern crate turtle;
use turtle::{Turtle, Point};

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
    let mut stack: Vec<(Point, f64)> = Vec::new();

    for i in instructions.chars() {
        match i {
            'F' => turtle.forward(dist),
            'f' => {turtle.pen_up(); turtle.forward(dist); turtle.pen_down()},
            '+' => turtle.left(rot),
            '-' => turtle.right(rot),
            '|' => turtle.right(180.0),
            '[' => {
                let tup = (turtle.position(), turtle.heading()); 
                stack.push(tup);
            },
            ']' => {
                let (pos, head) = stack.pop().unwrap(); 
                turtle.pen_up(); 
                turtle.go_to(pos); 
                turtle.set_heading(head); 
                turtle.pen_down();
            },
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
        turtle.drawing_mut().set_center(((0.0 - lX - sX) / 2.0, (0.0 - sY - lY) / 2.0));
        turtle.drawing_mut().set_size([(lX + (0.0 - sX) + 50.0) as u32, (lY + (0.0 - sY) + 50.0) as u32]);
    }
    turtle.drawing_mut().set_size([(lX + (0.0 - sX) + 100.0) as u32, (lY + (0.0 - sY) + 100.0) as u32]);
}

fn main() {
    let mut turtle = Turtle::new();
    turtle.drawing_mut().set_title("L-System Turtle!");
    turtle.set_speed("instant");

    let axiom = "A";
    let steps = 6;
    let rules = |x| match x {
        'A' => "F-[[A]+A]+F{[+FA]}-A",
        'B' => "A",
        'F' => "FF",
        'f' => "f",
        '+' => "+",
        '-' => "-",
        '|' => "|",
        '[' => "[",
        ']' => "]",
        '{' => "{",
        '}' => "}",
        _ => ""
    };

    instruct_turtle(&mut turtle, l_system(axiom, steps, &rules), 5.0, 22.5);
    turtle.drawing().save_svg("a.svg")
}
