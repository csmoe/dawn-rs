use dawn_codegen::{
    Annotation, DawnJsonParser, Definition, ExtensibleType, LengthValue, ReturnType,
};

#[test]
fn test_parse_file() {
    let parser = DawnJsonParser::parse_file("/Users/mac/repo/dawn-rs/target/dawn.json").unwrap();
    let codegen = parser.codegen();
    std::fs::write("/Users/mac/repo/dawn-rs/target/dawn.rs", codegen).unwrap();
}
