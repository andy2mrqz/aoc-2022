use std::process::Command;

fn main() {
    println!();
    println!("🎄 Advent of Code 🎄");
    println!();

    for day in 1..=6 {
        let day = format!("{:02}", day);
        let cmd = Command::new("cargo")
            .args(["run", "--bin", &day])
            .output()
            .unwrap();

        println!("Day {}", day);
        let output = String::from_utf8(cmd.stdout).unwrap();
        if output.is_empty() {
            println!("Not implemented")
        } else {
            println!("{}", output.trim());
        }
        println!();
    }
}
