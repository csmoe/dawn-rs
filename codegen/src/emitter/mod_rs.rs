pub(crate) fn emit() -> String {
    let content = r#"#![allow(dead_code, unused_imports)]

mod enums;
mod structs;
mod extensions;
mod objects;
mod callbacks;
mod functions;
mod constants;

pub use enums::*;
pub use structs::*;
pub use extensions::*;
pub use objects::*;
pub use callbacks::*;
pub use functions::*;
pub use constants::*;
"#;

    content
        .replacen(
            "#![allow(dead_code, unused_imports)]",
            "#[allow(dead_code, unused_imports)]",
            1,
        )
        .to_string()
}
