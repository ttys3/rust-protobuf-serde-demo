use std::io::Result;
fn main() -> Result<()> {
    let mut config = prost_build::Config::default();
    
    config.type_attribute("snazzy.items.Shirt", "#[derive(serde::Serialize, serde::Deserialize)]").
    type_attribute("snazzy.items.Shirt", "#[serde(default)]").
    out_dir("src/proto").
    compile_protos(&["src/proto/items.proto"], &["src/"])?;

    Ok(())
}
