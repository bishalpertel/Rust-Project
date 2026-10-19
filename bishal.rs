fn main() {
    let mut coding_skills = 100;
    let mut attempts = 0;

    println!("Hey LinkedIn! I've started learning Rust!");
    println!("And no, it's NOT on Duolingo.");

    for try_number in 1..=5 {
        attempts += 1;
        println!("Attempt {}: Fighting the compiler...", try_number);
    }

    coding_skills += 1;

    println!("\nAttempts made: {}", attempts);
    println!("Initial coding skills: 100");
    println!("Updated coding skills: {}", coding_skills);
    println!("Confidence level: 1000");
    println!("Compiler approval: 0");

    println!("\nBackend developer comeback: loading...");
    //println!("\nopps..ignore this comment....");
    println!("Developer sanity remaining: 0%");


    println!("\n100 + 1 = 101. We're so back!");
}