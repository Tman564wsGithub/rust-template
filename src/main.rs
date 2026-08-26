use eyre::Result;

mod lib;

fn main() -> Result<()> {
    lib::greet("{{project-name}}");

    Ok(())
}
