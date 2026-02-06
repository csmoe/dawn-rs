use crate::parser::{
    BitmaskDef, CallbackDef, CallbackFunctionDef, CallbackInfoDef, ConstantDef, DawnApi, EnumDef,
    FunctionDef, FunctionPointerDef, NativeType, ObjectDef, StructureDef, TypedefDef,
};

#[derive(Debug, Clone)]
pub struct ApiModel {
    pub enums: Vec<EnumModel>,
    pub bitmasks: Vec<BitmaskModel>,
    pub structures: Vec<StructureModel>,
    pub objects: Vec<ObjectModel>,
    pub functions: Vec<FunctionModel>,
    pub callbacks: Vec<CallbackModel>,
    pub callback_functions: Vec<CallbackFunctionModel>,
    pub callback_infos: Vec<CallbackInfoModel>,
    pub constants: Vec<ConstantModel>,
    pub natives: Vec<NativeModel>,
    pub typedefs: Vec<TypedefModel>,
    pub function_pointers: Vec<FunctionPointerModel>,
}

#[derive(Debug, Clone)]
pub struct EnumModel {
    pub name: String,
    pub def: EnumDef,
}

#[derive(Debug, Clone)]
pub struct BitmaskModel {
    pub name: String,
    pub def: BitmaskDef,
}

#[derive(Debug, Clone)]
pub struct StructureModel {
    pub name: String,
    pub def: StructureDef,
}

#[derive(Debug, Clone)]
pub struct ObjectModel {
    pub name: String,
    pub def: ObjectDef,
}

#[derive(Debug, Clone)]
pub struct FunctionModel {
    pub name: String,
    pub def: FunctionDef,
}

#[derive(Debug, Clone)]
pub struct CallbackModel {
    pub name: String,
    pub def: CallbackDef,
}

#[derive(Debug, Clone)]
pub struct CallbackFunctionModel {
    pub name: String,
    pub def: CallbackFunctionDef,
}

#[derive(Debug, Clone)]
pub struct CallbackInfoModel {
    pub name: String,
    pub def: CallbackInfoDef,
}

#[derive(Debug, Clone)]
pub struct ConstantModel {
    pub name: String,
    pub def: ConstantDef,
}

#[derive(Debug, Clone)]
pub struct NativeModel {
    pub name: String,
    pub def: NativeType,
}

#[derive(Debug, Clone)]
pub struct TypedefModel {
    pub name: String,
    pub def: TypedefDef,
}

#[derive(Debug, Clone)]
pub struct FunctionPointerModel {
    pub name: String,
    pub def: FunctionPointerDef,
}

impl ApiModel {
    pub fn from_api(api: &DawnApi) -> Self {
        let mut enums = Vec::new();
        let mut bitmasks = Vec::new();
        let mut structures = Vec::new();
        let mut objects = Vec::new();
        let mut functions = Vec::new();
        let mut callbacks = Vec::new();
        let mut callback_functions = Vec::new();
        let mut callback_infos = Vec::new();
        let mut constants = Vec::new();
        let mut natives = Vec::new();
        let mut typedefs = Vec::new();
        let mut function_pointers = Vec::new();

        for (name, def) in &api.definitions {
            match def {
                crate::parser::Definition::Enum(enum_def) => enums.push(EnumModel {
                    name: name.clone(),
                    def: enum_def.clone(),
                }),
                crate::parser::Definition::Bitmask(bitmask_def) => bitmasks.push(BitmaskModel {
                    name: name.clone(),
                    def: bitmask_def.clone(),
                }),
                crate::parser::Definition::Structure(struct_def) => {
                    structures.push(StructureModel {
                        name: name.clone(),
                        def: struct_def.clone(),
                    })
                }
                crate::parser::Definition::Object(object_def) => objects.push(ObjectModel {
                    name: name.clone(),
                    def: object_def.clone(),
                }),
                crate::parser::Definition::Function(func_def) => functions.push(FunctionModel {
                    name: name.clone(),
                    def: func_def.clone(),
                }),
                crate::parser::Definition::Callback(callback_def) => {
                    callbacks.push(CallbackModel {
                        name: name.clone(),
                        def: callback_def.clone(),
                    })
                }
                crate::parser::Definition::CallbackFunction(callback_def) => callback_functions
                    .push(CallbackFunctionModel {
                        name: name.clone(),
                        def: callback_def.clone(),
                    }),
                crate::parser::Definition::CallbackInfo(callback_info_def) => {
                    callback_infos.push(CallbackInfoModel {
                        name: name.clone(),
                        def: callback_info_def.clone(),
                    })
                }
                crate::parser::Definition::Constant(const_def) => constants.push(ConstantModel {
                    name: name.clone(),
                    def: const_def.clone(),
                }),
                crate::parser::Definition::Native(native_def) => natives.push(NativeModel {
                    name: name.clone(),
                    def: native_def.clone(),
                }),
                crate::parser::Definition::Typedef(typedef_def) => typedefs.push(TypedefModel {
                    name: name.clone(),
                    def: typedef_def.clone(),
                }),
                crate::parser::Definition::FunctionPointer(fp_def) => {
                    function_pointers.push(FunctionPointerModel {
                        name: name.clone(),
                        def: fp_def.clone(),
                    })
                }
            }
        }

        enums.sort_by(|a, b| a.name.cmp(&b.name));
        bitmasks.sort_by(|a, b| a.name.cmp(&b.name));
        structures.sort_by(|a, b| a.name.cmp(&b.name));
        objects.sort_by(|a, b| a.name.cmp(&b.name));
        functions.sort_by(|a, b| a.name.cmp(&b.name));
        callbacks.sort_by(|a, b| a.name.cmp(&b.name));
        callback_functions.sort_by(|a, b| a.name.cmp(&b.name));
        callback_infos.sort_by(|a, b| a.name.cmp(&b.name));
        constants.sort_by(|a, b| a.name.cmp(&b.name));
        natives.sort_by(|a, b| a.name.cmp(&b.name));
        typedefs.sort_by(|a, b| a.name.cmp(&b.name));
        function_pointers.sort_by(|a, b| a.name.cmp(&b.name));

        Self {
            enums,
            bitmasks,
            structures,
            objects,
            functions,
            callbacks,
            callback_functions,
            callback_infos,
            constants,
            natives,
            typedefs,
            function_pointers,
        }
    }
}
