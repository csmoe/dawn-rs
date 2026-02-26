use crate::api_model::ApiModel;

use crate::emitter::core::{TypeIndex, build_constructor_map, emit_object};

pub(crate) fn emit(model: &ApiModel, c_prefix: &str) -> String {
    let mut out = String::new();
    out.push_str(
        r#"#![allow(dead_code, unused_imports)]

use crate::generated::*;
use crate::ffi;
"#,
    );

    let index = TypeIndex::new(model);
    let constructor_map = build_constructor_map(model);
    for o in &model.objects {
        let constructor = constructor_map.get(&o.name);
        out.push_str(&emit_object(o, constructor, model, &index, c_prefix));
    }

    out
}
