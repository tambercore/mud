pub struct Derivation {
    pub contents: String,
    pub Id: String,
}

pub fn print_derivations(lines: &Vec<Derivation>) {
    println!("Derivations: ");
    for line in lines {
        println!("{} : {}", line.Id, line.contents);
    }
}
