use crate::api_model::{
    ApiModel, BitmaskModel, CallbackFunctionModel, CallbackInfoModel, CallbackModel, ConstantModel,
    EnumModel, FunctionModel, ObjectModel, StructureModel,
};

pub trait ApiVisitor {
    fn begin(&mut self, _model: &ApiModel) {}
    fn visit_enum(&mut self, _e: &EnumModel) {}
    fn visit_bitmask(&mut self, _b: &BitmaskModel) {}
    fn visit_struct(&mut self, _s: &StructureModel) {}
    fn visit_object(&mut self, _o: &ObjectModel) {}
    fn visit_function(&mut self, _f: &FunctionModel) {}
    fn visit_callback(&mut self, _c: &CallbackModel) {}
    fn visit_callback_function(&mut self, _c: &CallbackFunctionModel) {}
    fn visit_callback_info(&mut self, _c: &CallbackInfoModel) {}
    fn visit_constant(&mut self, _c: &ConstantModel) {}
    fn end(&mut self, _model: &ApiModel) {}
}

pub fn walk_model<V: ApiVisitor>(visitor: &mut V, model: &ApiModel) {
    visitor.begin(model);

    for e in &model.enums {
        visitor.visit_enum(e);
    }
    for b in &model.bitmasks {
        visitor.visit_bitmask(b);
    }
    for s in &model.structures {
        visitor.visit_struct(s);
    }
    for c in &model.callback_functions {
        visitor.visit_callback_function(c);
    }
    for c in &model.callback_infos {
        visitor.visit_callback_info(c);
    }
    for o in &model.objects {
        visitor.visit_object(o);
    }
    for f in &model.functions {
        visitor.visit_function(f);
    }
    for c in &model.callbacks {
        visitor.visit_callback(c);
    }
    for c in &model.constants {
        visitor.visit_constant(c);
    }

    visitor.end(model);
}
