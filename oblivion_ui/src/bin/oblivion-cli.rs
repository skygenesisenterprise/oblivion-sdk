use clap::{Parser, Subcommand};
use std::fs;
use std::path::Path;

#[derive(Parser)]
#[command(name = "oblivion-cli")]
#[command(about = "CLI tool for Oblivion UI SDK")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new Oblivion UI project
    New {
        /// Name of the project
        name: String,
    },
    /// Build the project
    Build,
    /// Run the project
    Run,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::New { name } => {
            create_project(&name);
        }
        Commands::Build => {
            println!("Building project...");
            // Run cargo build
            std::process::Command::new("cargo")
                .arg("build")
                .status()
                .expect("Failed to build");
        }
        Commands::Run => {
            println!("Running project...");
            std::process::Command::new("cargo")
                .arg("run")
                .status()
                .expect("Failed to run");
        }
    }
}

fn create_project(name: &str) {
    let project_dir = Path::new(name);

    if project_dir.exists() {
        eprintln!("Project {} already exists!", name);
        return;
    }

    fs::create_dir(project_dir).expect("Failed to create project directory");

    // Create Cargo.toml
    let cargo_toml = format!(r#"[package]
name = "{}"
version = "0.1.0"
edition = "2021"

[dependencies]
oblivion_ui = {{ path = "../oblivion_ui" }}
"#, name);

    fs::write(project_dir.join("Cargo.toml"), cargo_toml).expect("Failed to write Cargo.toml");

    // Create src/main.rs
    let main_rs = "fn main() {\n    println!(\"Hello, Oblivion UI!\");\n}\n";

    fs::create_dir(project_dir.join("src")).expect("Failed to create src directory");
    fs::write(project_dir.join("src/main.rs"), main_rs).expect("Failed to write main.rs");

    println!("Project {} created successfully!", name);
    println!("Run 'cd {} && cargo run' to start", name);
}
"#;

    fs::write(project_dir.join("src/main.rs"), main_rs).expect("Failed to write main.rs");

    println!("Project {} created successfully!", name);
    println!("Run 'cd {} && cargo run' to start", name);
}