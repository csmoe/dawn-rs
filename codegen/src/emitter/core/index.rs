use crate::api_model::{ApiModel, CallbackFunctionModel, FunctionModel, StructureModel};
use crate::emitter::core::is_char_string_list;
use crate::emitter::core::names::{enum_variant_name_camel, shouty_snake_case_name, type_name};
use crate::parser::LengthValue;
use std::collections::{HashMap, HashSet};

pub(crate) struct TypeIndex {
    objects: HashMap<String, ()>,
    enums: HashMap<String, ()>,
    bitmasks: HashMap<String, ()>,
    structs: HashMap<String, bool>,
    structs_defaultable: HashMap<String, ()>,
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

        let mut structs_defaultable = HashMap::new();
        loop {
            let mut changed = false;
            for s in &model.structures {
                if structs_defaultable.contains_key(&s.name) {
                    continue;
                }
                let length_fields = s
                    .def
                    .members
                    .iter()
                    .filter_map(|member| match &member.length {
                        Some(LengthValue::String(name)) => Some(name.as_str()),
                        _ => None,
                    })
                    .collect::<HashSet<_>>();
                let defaultable = s.def.is_output()
                    || s.def.members.iter().all(|member| {
                        let length_has_default =
                            member.length.as_ref().is_some_and(|length| match length {
                                LengthValue::String(name) => s
                                    .def
                                    .members
                                    .iter()
                                    .find(|candidate| candidate.name == *name)
                                    .is_some_and(|count| count.default.is_some()),
                                LengthValue::Number(_) => false,
                            });
                        length_fields.contains(member.name.as_str())
                            || member.optional
                            || member.default.is_some()
                            || length_has_default
                            || callback_infos.contains_key(&member.member_type)
                            || (member.annotation.is_value()
                                && structs_defaultable.contains_key(&member.member_type))
                    });
                if defaultable {
                    structs_defaultable.insert(s.name.clone(), ());
                    changed = true;
                }
            }
            if !changed {
                break;
            }
        }

        Self {
            objects,
            enums,
            bitmasks,
            structs,
            structs_defaultable,
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

    pub(crate) fn struct_defaultable(&self, name: &str) -> bool {
        self.structs_defaultable.contains_key(name)
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
pub(crate) fn build_callback_function_map(
    model: &ApiModel,
) -> HashMap<String, CallbackFunctionModel> {
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
    if !s.def.is_output() {
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
