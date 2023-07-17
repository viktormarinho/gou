

#[derive(Debug, Clone, clap::Parser)]
pub struct Fix {
    #[clap(value_delimiter = ' ', num_args = 1..)]
    message: Vec<String>,
    #[clap(short, long, default_value = "0")]
    prates: u8
}

impl Fix {
    pub fn run(self) {
        let commit_message = self.message.join(" ");

        println!("Commit message: {}", commit_message);
        println!("Prates: {}", self.prates);
    }
}