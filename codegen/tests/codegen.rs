use dawn_codegen::{
    Annotation, DawnJsonParser, Definition, ExtensibleType, LengthValue, ReturnType, codegen,
};

#[test]
fn test_parse_file() {
    let ret = DawnJsonParser::parse_file("/Users/mac/repo/dawn-rs/target/dawn.json")
        .unwrap()
        .codegen();
    std::fs::write("/Users/mac/repo/dawn-rs/target/dawn.rs", ret).unwrap();
    codegen("/Users/mac/repo/dawn-rs/target/c-api.rs");
}
