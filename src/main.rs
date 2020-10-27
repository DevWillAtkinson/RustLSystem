// Compute a Lindenmayer system given an axiom, a number of steps and rules

fn l_system<'a>(axiom: &str, steps: u32, rules: &dyn Fn(char) -> &'a str) -> String {
    let mut input = String::from(axiom);
    for _ in 0..steps {
        let out = input.chars().map(rules).collect();
        input = out;
    }

    input.to_string()
}



fn main() {
    
    let axiom = "A";
    let steps = 7;
    let rules = |x| match x {
        'A' => "AB",
        'B' => "A",
        _ => ""
    };

    let out = l_system(axiom, steps, &rules);
    println!("{}", out);

    let axiom = "F";
    let steps = 2;
    let rules = |x| match x {
        'F' => "F+F−F−F+F",
        '+' => "+",
        '−' => "−",
        _ => ""
    };

    let out = l_system(axiom, steps, &rules);
    println!("{}", out);
}
