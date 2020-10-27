// Compute a Lindenmayer system given an axiom, a number of steps and rules

fn lSystem(axiom: &str, steps: u32, rules: &[&str]) -> String {
    let mut input = String::from(axiom);
    for i in 0..steps {
        let out = input.chars().map(|letter| {
            match letter {
                'A' => "AB",
                'B' => "A",
                _ => ""
            }
        }).collect();
        input = out;
    }

    input.to_string()
}



fn main() {
    
    let axiom = "A";
    let steps = 5;
    let rules = ["AB", "A"];

    let out = lSystem(axiom, steps, &rules);
    println!("{}", out);
}
