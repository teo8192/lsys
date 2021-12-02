mod lsystem;

use std::error::Error;

use lsystem::LSystem;

fn main() -> Result<(), Box<dyn Error>> {
    let lsys = LSystem::from_str("F F->GF G->F")?;
    for word in lsys.take(5) {
        println!("{:?}", word);
    }
    Ok(())
}
