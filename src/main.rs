mod lsystem;

use std::error::Error;

use lsystem::LSystem;

fn main() -> Result<(), Box<dyn Error>> {
    println!("{:?}", LSystem::from_str("F F->FG G->F")?);
    Ok(())
}
