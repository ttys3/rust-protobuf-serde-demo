use serde_json::json;

use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    println!("Hello, world!");
    let shirt = json!(create_large_shirt("red".to_string()));
    println!("shirt={}", shirt);
}

// Include the `items` module, which is generated from items.proto.
// https://doc.rust-lang.org/cargo/reference/environment-variables.html
pub mod items {
    // include!(concat!(env!("OUT_DIR"), "/snazzy.items.rs"));
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/proto/snazzy.items.rs"
    ));
}

pub fn create_large_shirt(color: String) -> items::Shirt {
    let mut shirt = items::Shirt::default();
    shirt.color = color;
    shirt.ts = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as i64;
    shirt.set_size(items::shirt::Size::Large);
    shirt
}
