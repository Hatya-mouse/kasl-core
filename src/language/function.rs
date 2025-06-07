use crate::Type;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct FunctionInfo {
    pub name: String,
    pub arguments: Vec<Type>,
    pub return_type: Type,
}

pub fn built_in_functions() -> HashMap<String, FunctionInfo> {
    let mut functions = HashMap::new();

    functions.insert(
        "sin".to_string(),
        FunctionInfo {
            name: "sin".to_string(),
            arguments: vec![Type::Float],
            return_type: Type::Float,
        },
    );

    functions.insert(
        "cos".to_string(),
        FunctionInfo {
            name: "cos".to_string(),
            arguments: vec![Type::Float],
            return_type: Type::Float,
        },
    );

    functions.insert(
        "tan".to_string(),
        FunctionInfo {
            name: "tan".to_string(),
            arguments: vec![Type::Float],
            return_type: Type::Float,
        },
    );

    functions.insert(
        "asin".to_string(),
        FunctionInfo {
            name: "asin".to_string(),
            arguments: vec![Type::Float],
            return_type: Type::Float,
        },
    );

    functions.insert(
        "acos".to_string(),
        FunctionInfo {
            name: "acos".to_string(),
            arguments: vec![Type::Float],
            return_type: Type::Float,
        },
    );

    functions.insert(
        "atan".to_string(),
        FunctionInfo {
            name: "atan".to_string(),
            arguments: vec![Type::Float],
            return_type: Type::Float,
        },
    );

    functions.insert(
        "abs".to_string(),
        FunctionInfo {
            name: "abs".to_string(),
            arguments: vec![Type::Float],
            return_type: Type::Float,
        },
    );

    functions.insert(
        "sgn".to_string(),
        FunctionInfo {
            name: "sgn".to_string(),
            arguments: vec![Type::Float],
            return_type: Type::Float,
        },
    );

    functions.insert(
        "min".to_string(),
        FunctionInfo {
            name: "min".to_string(),
            arguments: vec![Type::Float, Type::Float],
            return_type: Type::Float,
        },
    );

    functions.insert(
        "max".to_string(),
        FunctionInfo {
            name: "max".to_string(),
            arguments: vec![Type::Float, Type::Float],
            return_type: Type::Float,
        },
    );

    functions.insert(
        "clamp".to_string(),
        FunctionInfo {
            name: "clamp".to_string(),
            arguments: vec![Type::Float, Type::Float, Type::Float],
            return_type: Type::Float,
        },
    );

    functions.insert(
        "pow".to_string(),
        FunctionInfo {
            name: "pow".to_string(),
            arguments: vec![Type::Float, Type::Float],
            return_type: Type::Float,
        },
    );

    functions.insert(
        "sqrt".to_string(),
        FunctionInfo {
            name: "sqrt".to_string(),
            arguments: vec![Type::Float],
            return_type: Type::Float,
        },
    );

    functions.insert(
        "log".to_string(),
        FunctionInfo {
            name: "log".to_string(),
            arguments: vec![Type::Float],
            return_type: Type::Float,
        },
    );

    functions.insert(
        "log2".to_string(),
        FunctionInfo {
            name: "log2".to_string(),
            arguments: vec![Type::Float],
            return_type: Type::Float,
        },
    );

    functions.insert(
        "log10".to_string(),
        FunctionInfo {
            name: "log10".to_string(),
            arguments: vec![Type::Float],
            return_type: Type::Float,
        },
    );

    functions.insert(
        "saw".to_string(),
        FunctionInfo {
            name: "saw".to_string(),
            arguments: vec![Type::Float],
            return_type: Type::Float,
        },
    );

    functions.insert(
        "tri".to_string(),
        FunctionInfo {
            name: "tri".to_string(),
            arguments: vec![Type::Float],
            return_type: Type::Float,
        },
    );

    functions.insert(
        "square".to_string(),
        FunctionInfo {
            name: "square".to_string(),
            arguments: vec![Type::Float],
            return_type: Type::Float,
        },
    );

    functions.insert(
        "rand".to_string(),
        FunctionInfo {
            name: "rand".to_string(),
            arguments: vec![],
            return_type: Type::Float,
        },
    );

    functions.insert(
        "sample".to_string(),
        FunctionInfo {
            name: "sample".to_string(),
            arguments: vec![],
            return_type: Type::Float,
        },
    );

    functions.insert(
        "sample_rate".to_string(),
        FunctionInfo {
            name: "sample_rate".to_string(),
            arguments: vec![],
            return_type: Type::Float,
        },
    );

    functions.insert(
        "bpm".to_string(),
        FunctionInfo {
            name: "bpm".to_string(),
            arguments: vec![],
            return_type: Type::Float,
        },
    );

    functions.insert(
        "time".to_string(),
        FunctionInfo {
            name: "time".to_string(),
            arguments: vec![],
            return_type: Type::Float,
        },
    );

    functions.insert(
        "phase".to_string(),
        FunctionInfo {
            name: "phase".to_string(),
            arguments: vec![],
            return_type: Type::Float,
        },
    );

    functions.insert(
        "mix".to_string(),
        FunctionInfo {
            name: "mix".to_string(),
            arguments: vec![Type::Float, Type::Float, Type::Float],
            return_type: Type::Float,
        },
    );

    functions.insert(
        "lerp".to_string(),
        FunctionInfo {
            name: "lerp".to_string(),
            arguments: vec![Type::Float, Type::Float, Type::Float],
            return_type: Type::Float,
        },
    );

    functions.insert(
        "pi".to_string(),
        FunctionInfo {
            name: "pi".to_string(),
            arguments: vec![],
            return_type: Type::Float,
        },
    );

    functions
}
