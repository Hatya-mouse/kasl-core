//
// Copyright 2025 Shuntaro Kasatani
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//

use crate::{Expression, Operator, Statement};
use cranelift_codegen::{
    entity::EntityRef,
    ir::{self, Block, InstBuilder, types},
};
use cranelift_frontend::{FunctionBuilder, Variable};
use cranelift_jit::JITModule;
use cranelift_module::Module;
use std::collections::HashMap;

pub struct Translator<'a> {
    pub builder: FunctionBuilder<'a>,
    pub variables: HashMap<String, Variable>,
    functions: HashMap<String, ir::FuncRef>,
    entry_block: Block,
    return_block: Block,
}

impl<'a> Translator<'a> {
    pub fn new(
        builder: FunctionBuilder<'a>,
        functions: HashMap<String, ir::FuncRef>,
        entry_block: Block,
        return_block: Block,
    ) -> Self {
        Translator {
            builder,
            variables: HashMap::new(),
            functions,
            entry_block,
            return_block,
        }
    }

    pub fn codegen_stmt(
        &mut self,
        inputs: &[String],
        outputs: &[String],
        statement: &Statement,
        module: &JITModule,
    ) {
        match statement {
            Statement::InputDeclaration(input_decl) => {
                let var = Variable::new(self.variables.len());
                let val = self.builder.block_params(self.entry_block)
                    [inputs.iter().position(|p| p == &input_decl.name).unwrap()];
                self.variables.insert(input_decl.name.clone(), var);

                let var_type = get_type(input_decl.value_type.clone(), module);

                self.builder.declare_var(var, var_type);
                self.builder.def_var(var, val);
            }
            Statement::OutputDeclaration(output_decl) => {
                let var = Variable::new(self.variables.len());
                let val = self.builder.block_params(self.return_block)
                    [outputs.iter().position(|p| p == &output_decl.name).unwrap()];
                self.variables.insert(output_decl.name.clone(), var);

                let var_type = get_type(output_decl.value_type.clone(), module);

                self.builder.declare_var(var, var_type);
                self.builder.def_var(var, val);
            }
            Statement::VariableDeclaration(var_decl) => {
                let var = Variable::new(self.variables.len());
                let val = self.codegen_expr(&var_decl.initial_value);
                self.variables.insert(var_decl.name.clone(), var);

                let var_type = get_type(var_decl.value_type.clone(), module);

                self.builder.declare_var(var, var_type);
                self.builder.def_var(var, val);
            }
            Statement::Assignment(assignment_stmt) => {
                if let Some(var) = self.variables.get(&assignment_stmt.target_name).cloned() {
                    let val = self.codegen_expr(&assignment_stmt.value);
                    self.builder.def_var(var, val);
                }
            }
            Statement::ForLoop(_for_loop_stmt) => {
                todo!()
            }
        }
    }

    pub fn codegen_expr(&mut self, expr: &Expression) -> ir::Value {
        match expr {
            Expression::IntLiteral(lit) => self.builder.ins().iconst(types::I32, *lit as i64),
            Expression::FloatLiteral(lit) => self.builder.ins().f32const(*lit),
            Expression::Identifier(id) => self.builder.use_var(self.variables[id]),
            Expression::BinaryOp { op, left, right } => {
                let left_val = self.codegen_expr(left);
                let right_val = self.codegen_expr(right);
                match op {
                    &Operator::Add => self.builder.ins().iadd(left_val, right_val),
                    &Operator::Subtract => self.builder.ins().isub(left_val, right_val),
                    &Operator::Multiply => self.builder.ins().imul(left_val, right_val),
                    &Operator::Divide => self.builder.ins().udiv(left_val, right_val),
                    &Operator::Modulo => self.builder.ins().urem(left_val, right_val),
                }
            }
            Expression::FunctionCall { name, arguments } => {
                let args = arguments
                    .iter()
                    .map(|arg| self.codegen_expr(arg))
                    .collect::<Vec<ir::Value>>();
                if let Some(function) = self.functions.get(name) {
                    let inst = self.builder.ins().call(*function, &args);
                    self.builder
                        .inst_results(inst)
                        .first()
                        .unwrap_or(&ir::Value::new(0))
                        .clone()
                } else {
                    ir::Value::new(0)
                }
            }
        }
    }

    pub fn get_returns(&self) -> Vec<ir::Value> {
        self.builder.block_params(self.return_block).to_vec()
    }
}

pub fn get_type(value_type: knodiq_engine::Type, module: &JITModule) -> types::Type {
    match value_type {
        knodiq_engine::Type::Int => types::I32,
        knodiq_engine::Type::Float => types::F32,
        knodiq_engine::Type::Array(_) => module.target_config().pointer_type(),
        knodiq_engine::Type::None => types::INVALID,
    }
}
