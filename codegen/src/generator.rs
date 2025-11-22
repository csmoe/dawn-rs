use std::fmt::format;

use heck::{ToPascalCase, ToSnakeCase};

use crate::{Annotation, FunctionDef, Method, ObjectDef, RecordMember, ReturnType};

struct Function {
    name: String,
    def: FunctionDef,
}

trait Codegen {
    fn codegen(&self) -> String;
}

impl Codegen for Function {
    fn codegen(&self) -> String {
        let FunctionDef {
            returns,
            args,
            comment,
            ..
        } = &self.def;
        let name = self.name.to_snake_case();
        let ret = returns.codegen();
        let comment = comment
            .as_ref()
            .map(|c| format!("/// {}", c))
            .unwrap_or_default();
        let args = args
            .iter()
            .map(|arg| arg.codegen())
            .collect::<Vec<_>>()
            .join(", ");

        format!(
            r#"
{comment}
pub fn {name}({args}) -> {ret} {{
}}
"#
        )
    }
}

impl Codegen for Option<ReturnType> {
    fn codegen(&self) -> String {
        match self {
            Some(ReturnType::Complex {
                return_type,
                optional,
            }) => {
                let ty = return_type.to_pascal_case();
                if *optional {
                    format!("Option<{ty}>")
                } else {
                    format!("{ty}")
                }
            }
            Some(ReturnType::Simple(return_type)) => {
                let ty = return_type.to_pascal_case();
                format!("{ty}")
            }
            None => "()".to_string(),
        }
    }
}

impl Codegen for RecordMember {
    fn codegen(&self) -> String {
        let RecordMember {
            name,
            member_type,
            annotation,
            length,
            optional,
            default,
            wire_is_data_only,
            skip_serialize,
            no_default,
            array_element_optional,
        } = &self;
        let name = name.to_snake_case();
        let ty = member_type.to_pascal_case();
        let annotation = annotation.codegen();

        format!("{name}: {annotation} {ty}")
    }
}

impl Codegen for Annotation {
    fn codegen(&self) -> String {
        match self {
            Annotation::MutPtr => "*mut ",
            Annotation::ConstPtr => "*const ",
            Annotation::ConstConstPtr => "*const *const ",
            Annotation::Value => "",
        }
        .to_string()
    }
}

impl Codegen for Method {
    fn codegen(&self) -> String {
        let Method {
            name,
            tags,
            returns,
            args,
            no_autolock,
            extensible,
        } = &self;
        let name = name.to_snake_case();
        let ret = returns.codegen();
        let args = args
            .iter()
            .map(|arg| arg.codegen())
            .collect::<Vec<String>>()
            .join(", ");
        format!("{name}(&self, {args}) -> {ret} {{}}")
    }
}

struct Object {
    name: String,
    def: ObjectDef,
}

impl Codegen for Object {
    fn codegen(&self) -> String {
        let ObjectDef {
            tags,
            methods,
            no_autolock,
        } = &self.def;
        let name = self.name.to_pascal_case();
        let methods = methods
            .iter()
            .map(|method| method.codegen())
            .collect::<Vec<String>>()
            .join("\n");
        let sync = if no_autolock.unwrap_or_default() {
            format!("unsafe impl !Send for {name} {{}}\nunsafe impl !Sync for {name} {{}}")
        } else {
            format!("unsafe impl Send for {name} {{}}\nunsafe impl Sync for {name} {{}}")
        };
        format!(
            r#"
pub struct {name};

{sync}

impl {name} {{
    {methods}
}}
"#
        )
    }
}
