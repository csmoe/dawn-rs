use crate::api_model::ApiModel;

pub struct GeneratedFiles {
    pub enums: String,
    pub structs: String,
    pub extensions: String,
    pub objects: String,
    pub callbacks: String,
    pub functions: String,
    pub constants: String,
}

pub fn generate_strings(model: &ApiModel) -> GeneratedFiles {
    let c_prefix = model.c_prefix.clone();
    let enums = format_rust_source(&crate::emitter::enums::emit(model, &c_prefix));
    let structs = format_rust_source(&crate::emitter::structs::emit(model, &c_prefix));
    let extensions = format_rust_source(&crate::emitter::extensions::emit(model, &c_prefix));
    let objects = format_rust_source(&crate::emitter::objects::emit(model, &c_prefix));
    let callbacks = format_rust_source(&crate::emitter::callbacks::emit(model, &c_prefix));
    let functions = format_rust_source(&crate::emitter::functions::emit(model, &c_prefix));
    let constants = format_rust_source(&crate::emitter::constants::emit(model));

    GeneratedFiles {
        enums,
        structs,
        extensions,
        objects,
        callbacks,
        functions,
        constants,
    }
}

pub fn format_rust_source(source: &str) -> String {
    match syn::parse_file(source) {
        Ok(file) => prettyplease::unparse(&file),
        Err(_) => source.to_string(),
    }
}
