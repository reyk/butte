use anyhow::Result;

fn main() -> Result<()> {
    for fbs in &["fbs/default_value.fbs", "fbs/greeter/greeter.fbs"] {
        butte_build::compile_fbs(fbs)?;
    }
    Ok(())
}
