//! Generate raw bindings from dawn.json
//!
//! [dawn codegen.md](https://github.com/google/dawn/blob/main/docs/dawn/codegen.md)
//!
//! TODO: download dawn api specs and binaries from https://github.com/google/dawn's releases.

#[derive(Debug)]
pub struct DawnApiSpec {
    ty: Type,
}

#[derive(Debug)]
pub enum Type {
    Native,
    Wire,
}

impl Type {
    fn spec_path(&self) -> std::path::PathBuf {
        match self {
            Type::Native => std::path::PathBuf::from(std::env::var("CARGO_TARGET_DIR").unwrap())
                .join("dawn.json"),
            Type::Wire => std::path::PathBuf::from(std::env::var("CARGO_TARGET_DIR").unwrap())
                .join("dawn_wire.json"),
        }
    }
}

impl DawnApiSpec {}

#[derive(Deserialize)]
struct Structure {
    members: Vec<Record>,
    extensible: bool,
    chained: bool,
    chained_roots: Vec<Self>,
}

#[derive(Deserialize)]
struct Record {
    name: String,
    r#type: String,
    annotation: Annotation,
    length: NonZeroU32,
    optional: bool,
    default: DefaultValue,
    wire_is_data_only: bool,
    no_default: bool,
}

#[derive(Debug, Default, Deserialize)]
#[strum]
enum Annotaion {
    /// value
    #[default]
    Value,
    /// *
    Ptr,
    /// const*
    ConstPtr,
}

enum DefaultValue {
    Number(u32),
    String(&'static str),
    Variant(&'static str),
}
