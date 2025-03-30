pub struct Derivation {
    pub contents: String,
    pub Id: String,
}

pub fn print_derivations(lines: &Derivations) {
    println!("Derivations: ");
    for line in &lines.contents {
        println!("{} : {}", line.Id, line.contents);
    }
}

pub struct Derivations {
    pub(crate) contents: Vec<Derivation>}

impl Derivations {
    pub fn find_id_by_contents(&self, contents: &str) -> Option<&str> {
        self.contents.iter()
            .find(|d| d.contents == contents)
            .map(|d| d.Id.as_str())
    }
}