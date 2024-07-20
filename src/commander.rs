use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Create a contact
    Create(CreateCommand),

    /// Edit a contact by ID
    Edit(EditCommand),
}

#[derive(Args)]
pub struct CreateCommand {
    #[arg(short, long, value_name = "First Name")]
    pub first_name: Option<String>,

    #[arg(short, long, value_name = "Last Name")]
    pub last_name: Option<String>,

    #[arg(short, long, value_name = "Display Name")]
    pub display_name: Option<String>,

    #[arg(short, long)]
    pub email: Option<String>,

    #[arg(short, long, value_name = "Phone")]
    pub phone_number: Option<String>,
}

#[derive(Args, Debug)]
pub struct EditCommand {
    /// ID of contact to edit
    id: i64,

    #[arg(short, long, value_name = "First Name")]
    first_name: Option<String>,

    #[arg(short, long, value_name = "Last Name")]
    last_name: Option<String>,

    #[arg(short, long, value_name = "Display Name")]
    display_name: Option<String>,

    #[arg(short, long)]
    email: Option<String>,

    #[arg(short, long, value_name = "Phone")]
    phone_number: Option<String>,
}
