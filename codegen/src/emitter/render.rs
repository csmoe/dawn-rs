use crate::api_model::ApiModel;
use std::fs;
use std::path::Path;

pub struct GeneratedFiles {
    pub enums: String,
    pub structs: String,
    pub extensions: String,
    pub objects: String,
    pub callbacks: String,
    pub functions: String,
    pub constants: String,
    pub mod_rs: String,
}

impl GeneratedFiles {
    pub fn write_to_dir(&self, out_dir: &Path) -> std::io::Result<()> {
        fs::create_dir_all(out_dir)?;
        fs::write(out_dir.join("enums.rs"), &self.enums)?;
        fs::write(out_dir.join("structs.rs"), &self.structs)?;
        fs::write(out_dir.join("extensions.rs"), &self.extensions)?;
        fs::write(out_dir.join("objects.rs"), &self.objects)?;
        fs::write(out_dir.join("callbacks.rs"), &self.callbacks)?;
        fs::write(out_dir.join("functions.rs"), &self.functions)?;
        fs::write(out_dir.join("constants.rs"), &self.constants)?;
        fs::write(out_dir.join("mod.rs"), &self.mod_rs)?;
        Ok(())
    }
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
    let mod_rs = format_rust_source(&crate::emitter::mod_rs::emit());

    GeneratedFiles {
        enums,
        structs,
        extensions,
        objects,
        callbacks,
        functions,
        constants,
        mod_rs,
    }
}

pub fn generate_to_dir(model: &ApiModel, out_dir: &Path) -> std::io::Result<()> {
    let files = generate_strings(model);
    files.write_to_dir(out_dir)
}

pub fn format_rust_source(source: &str) -> String {
    match syn::parse_file(source) {
        Ok(file) => prettyplease::unparse(&file),
        Err(_) => source.to_string(),
    }
}
