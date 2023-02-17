use anyhow::Result;

pub fn config_env_var(name: &str) -> Result<String> {
    let out = std::env::var(name)?;
    Ok(out)
}
