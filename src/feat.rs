
#[derive(Debug, Clone, clap::Parser)]
pub struct Feat {
}

impl Feat {
    pub fn run(self) {
        println!("Feat");
    }
}