

#[derive(Debug, Clone, clap::Parser)]
pub struct Refactor {
}

impl Refactor {
    pub fn run(self) {
        println!("Prates refactor");
    }
}