use crate::api_model::{ApiModel, CallbackFunctionModel, FunctionModel, StructureModel};
use std::collections::HashMap;
use crate::emitter::core::names::{enum_variant_name_camel, shouty_snake_case_name, type_name};
use crate::emitter::core::is_char_string_list;

pub(crate) struct TypeIndex {
    objects: HashMap<String, ()>,
    enums: HashMap<String, ()>,
    bitmasks: HashMap<String, ()>,
    structs: HashMap<String, bool>,
    callback_infos: HashMap<String, ()>,
    structs_need_free: HashMap<String, ()>,
}

impl TypeIndex {
    pub(crate) fn new(model: &ApiModel) -> Self {
        let mut objects = HashMap::new();
        let mut enums = HashMap::new();
        let mut bitmasks = HashMap::new();
        let mut structs = HashMap::new();
        let mut callback_infos = HashMap::new();
        let mut structs_need_free = HashMap::new();

        for o in &model.objects {
            objects.insert(o.name.clone(), ());
        }
        for e in &model.enums {
            enums.insert(e.name.clone(), ());
        }
        for b in &model.bitmasks {
            bitmasks.insert(b.name.clone(), ());
        }
        for s in &model.structures {
            structs.insert(s.name.clone(), s.def.extensible.is_extensible());
            if struct_needs_free_members(s) {
                structs_need_free.insert(s.name.clone(), ());
            }
        }
        for c in &model.callback_infos {
            callback_infos.insert(c.name.clone(), ());
        }

        Self {
            objects,
            enums,
            bitmasks,
            structs,
            callback_infos,
            structs_need_free,
        }
    }

    pub(crate) fn is_object(&self, name: &str) -> bool {
        self.objects.contains_key(name)
    }

    pub(crate) fn is_enum(&self, name: &str) -> bool {
        self.enums.contains_key(name)
    }

    pub(crate) fn is_bitmask(&self, name: &str) -> bool {
        self.bitmasks.contains_key(name)
    }

    pub(crate) fn struct_extensible(&self, name: &str) -> Option<bool> {
        self.structs.get(name).copied()
    }

    pub(crate) fn is_callback_info(&self, name: &str) -> bool {
        self.callback_infos.contains_key(name)
    }

    pub(crate) fn struct_needs_free_members(&self, name: &str) -> bool {
        self.structs_need_free.contains_key(name)
    }
}
pub(crate) fn build_constant_map(model: &ApiModel) -> HashMap<String, String> {
    let mut map = HashMap::new();
    for c in &model.constants {
        map.insert(c.name.clone(), shouty_snake_case_name(&c.name));
    }
    map
}
pub(crate) fn build_constructor_map(model: &ApiModel) -> HashMap<String, FunctionModel> {
    let mut map = HashMap::new();
    for func in &model.functions {
        if let Some(obj_name) = func.name.strip_prefix("create ") {
            map.insert(obj_name.to_string(), func.clone());
        }
    }
    map
}
pub(crate) fn build_callback_info_map(model: &ApiModel) -> HashMap<String, String> {
    let mut map = HashMap::new();
    for info in &model.callback_infos {
        if let Some(callback_member) = info
            .def
            .members
            .iter()
            .find(|member| member.name == "callback")
        {
            map.insert(info.name.clone(), callback_member.member_type.clone());
        }
    }
    map
}
pub(crate) fn build_callback_function_map(model: &ApiModel) -> HashMap<String, CallbackFunctionModel> {
    let mut map = HashMap::new();
    for callback in &model.callback_functions {
        map.insert(callback.name.clone(), callback.clone());
    }
    map
}
pub(crate) fn build_callback_info_mode_map(model: &ApiModel) -> HashMap<String, bool> {
    let mut map = HashMap::new();
    for info in &model.callback_infos {
        let has_mode = info.def.members.iter().any(|member| member.name == "mode");
        map.insert(info.name.clone(), has_mode);
    }
    map
}
pub(crate) fn build_stype_map(model: &ApiModel, c_prefix: &str) -> HashMap<String, String> {
    let mut map = HashMap::new();
    let stype_enum_name = "s type";
    let enum_name = type_name(stype_enum_name);
    if let Some(stype_enum) = model.enums.iter().find(|e| e.name == stype_enum_name) {
        for value in &stype_enum.def.values {
            let variant = enum_variant_name_camel(&value.name);
            map.insert(value.name.clone(), format!(r#"{enum_name}::{variant}"#));
        }
    }
    let _ = c_prefix;
    map
}

fn struct_needs_free_members(s: &StructureModel) -> bool {
    let is_out = s.def.extensible.is_output()
        || s.def.chained.as_deref() == Some("out")
        || s.def.out == Some(true);
    if !is_out {
        return false;
    }
    s.def.members.iter().any(|member| {
        if member.member_type == "string view" || is_char_string_list(member) {
            return true;
        }
        if member.length.is_some() {
            return true;
        }
        !member.annotation.is_value()
    })
}
