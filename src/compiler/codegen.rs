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

use crate::{Expression, Operator, Statement, SymbolInfo, TYPE_FLOAT, TYPE_INT};
use cranelift_codegen::{
    entity::EntityRef,
    ir::{self, Block, BlockArg, InstBuilder, types},
};
use cranelift_frontend::{FunctionBuilder, Variable};
use cranelift_jit::JITModule;
use cranelift_module::Module;
use knodiq_engine::graph::value::type_of;
use std::collections::HashMap;

pub struct Translator<'a> {
    pub builder: FunctionBuilder<'a>,
    pub variables: HashMap<String, (Variable, knodiq_engine::Type)>,
    pub return_vars: Vec<String>,
    pub functions: HashMap<String, ir::FuncRef>,
    entry_block: Block,
    input_ptr: ir::Value,
    output_ptr: ir::Value,
    var_counter: usize,
}

impl<'a> Translator<'a> {
    /// Creates a new `Translator` instance with the given function builder, functions, and entry block.
    pub fn new(
        mut builder: FunctionBuilder<'a>,
        functions: HashMap<String, ir::FuncRef>,
        entry_block: Block,
    ) -> Self {
        let input_ptr = builder.ins().iconst(TYPE_INT, 0);
        let output_ptr = builder.ins().iconst(TYPE_INT, 0);

        Translator {
            builder,
            variables: HashMap::new(),
            return_vars: Vec::new(),
            functions,
            entry_block,
            input_ptr,
            output_ptr,
            var_counter: 0,
        }
    }

    /// Sets up the array interface for the function.
    pub fn setup_array_interface(
        &mut self,
        inputs: &Vec<SymbolInfo>,
        outputs: &Vec<SymbolInfo>,
        module: &JITModule,
    ) {
        let params = self.builder.block_params(self.entry_block);
        self.input_ptr = params[0];
        self.output_ptr = params[1];

        let mut input_offset = 0;
        for input_info in inputs {
            let var = self.new_var();
            let val_type = get_ir_type(&input_info.value_type, module);

            let offset = self.builder.ins().iconst(TYPE_INT, input_offset);
            let addr = self.builder.ins().iadd(self.input_ptr, offset);
            let val = self
                .builder
                .ins()
                .load(val_type, ir::MemFlags::new(), addr, 0);

            self.variables.insert(
                input_info.name.clone(),
                (var, input_info.value_type.clone()),
            );
            self.builder.declare_var(var, val_type);
            self.builder.def_var(var, val);

            // Calculate the offset for the next element
            input_offset += val_type.bytes() as i64;
        }

        for output_info in outputs {
            let var = self.new_var();
            let val_type = get_ir_type(&output_info.value_type, module);

            self.variables.insert(
                output_info.name.clone(),
                (var, output_info.value_type.clone()),
            );
            self.return_vars.push(output_info.name.clone());
            self.builder.declare_var(var, val_type);

            let default_val = match val_type {
                TYPE_INT => self.builder.ins().iconst(TYPE_INT, 0),
                types::F32 => self.builder.ins().f32const(0.0),
                _ => self.builder.ins().iconst(val_type, 0),
            };

            self.builder.def_var(var, default_val);
        }
    }

    /// Finalizes the array interface by storing the output values into the output pointer.
    pub fn finalize_array_interface(&mut self, output_names: &[String], module: &JITModule) {
        let mut output_offset = 0;
        for output_name in output_names.iter() {
            if let Some((var, val_type)) = self.variables.get(output_name) {
                let val = self.builder.use_var(*var);
                let offset = self.builder.ins().iconst(TYPE_INT, output_offset);
                let addr = self.builder.ins().iadd(self.output_ptr, offset);
                self.builder.ins().store(ir::MemFlags::new(), val, addr, 0);

                // Calculate the offset for the next element
                output_offset += get_ir_type(val_type, module).bytes() as i64;
            }
        }

        self.builder.ins().return_(&[]);
    }

    /// Generates code for a statement.
    pub fn codegen_stmt(&mut self, statement: &Statement, module: &JITModule) {
        match statement {
            Statement::VariableDeclaration(var_decl) => {
                let var = self.new_var();
                let (val, val_type) = self.codegen_expr(&var_decl.initial_value, module);

                self.variables
                    .insert(var_decl.name.clone(), (var, var_decl.value_type.clone()));

                self.builder
                    .declare_var(var, get_ir_type(&val_type, module));
                self.builder.def_var(var, val);
            }
            Statement::Assignment(assignment_stmt) => {
                let (val, val_type) = self.codegen_expr(&assignment_stmt.value, module);
                let val_ir_type = get_ir_type(&val_type, module);

                // Check if the variable already exists
                if let Some((existing_var, existing_type)) =
                    self.variables.get(&assignment_stmt.target_name)
                {
                    let existing_type = get_ir_type(existing_type, module);
                    if existing_type == val_ir_type {
                        // If the types are the same, reuse the existing variable
                        self.builder.def_var(*existing_var, val);
                    } else {
                        // If the types are different, create a new variable
                        let new_var = self.new_var();

                        self.variables
                            .insert(assignment_stmt.target_name.clone(), (new_var, val_type));
                        self.builder.declare_var(new_var, val_ir_type);
                        self.builder.def_var(new_var, val);
                    }
                }
            }
            Statement::ForLoop(_for_loop_stmt) => {
                // self.codegen_loop(for_loop_stmt.count, for_loop_stmt.body)
            }
            _ => return,
        }
    }

    /// Converts a `knodiq_engine::Value` to an IR value.
    pub fn value_as_ir(&mut self, value: &knodiq_engine::Value, module: &JITModule) -> ir::Value {
        match value {
            knodiq_engine::Value::Int(i) => self.builder.ins().iconst(TYPE_INT, *i as i64),
            knodiq_engine::Value::Float(f) => self.builder.ins().f32const(*f),
            knodiq_engine::Value::Array(arr) => {
                let vals = arr.iter().map(|v| self.value_as_ir(v, module)).collect();
                self.vec_as_array(vals, module.target_config().pointer_type(), module)
            }
        }
    }

    /// Generates code for an expression and returns the resulting IR value and its type.
    pub fn codegen_expr(
        &mut self,
        expr: &Expression,
        module: &JITModule,
    ) -> (ir::Value, knodiq_engine::Type) {
        match expr {
            Expression::IntLiteral(lit) => (
                self.builder.ins().iconst(TYPE_INT, *lit as i64),
                knodiq_engine::Type::Int,
            ),
            Expression::FloatLiteral(lit) => (
                self.builder.ins().f32const(*lit),
                knodiq_engine::Type::Float,
            ),
            Expression::Identifier(id) => {
                let (var, var_type) = self.variables.get(id).expect("Variable not found");
                (self.builder.use_var(*var), var_type.clone())
            }
            Expression::BinaryOp {
                op,
                left,
                right,
                left_type: _,
                right_type: _,
            } => {
                let (left_val, left_type) = self.codegen_expr(left, module);
                let (right_val, right_type) = self.codegen_expr(right, module);
                let op_type = type_of(&left_type, &right_type);
                (
                    self.codegen_op(
                        op,
                        left_val,
                        right_val,
                        left_type.clone(),
                        right_type.clone(),
                        module,
                    ),
                    op_type,
                )
            }
            Expression::FunctionCall { name, arguments } => {
                let mut args = Vec::new();
                let mut arg_types = Vec::new();
                for arg in arguments {
                    let processed_expr = self.codegen_expr(arg, module);
                    args.push(processed_expr.0);
                    arg_types.push(processed_expr.1);
                }

                self.codegen_broadcast_op(
                    |vals, _types, slf, _module| slf.call_function(name, vals)[0],
                    args,
                    arg_types,
                    module,
                )
            }
        }
    }

    /// Generates code for a binary operation based on the operator and types of the left and right values.
    pub fn codegen_op(
        &mut self,
        op: &Operator,
        left: ir::Value,
        right: ir::Value,
        left_type: knodiq_engine::Type,
        right_type: knodiq_engine::Type,
        module: &JITModule,
    ) -> ir::Value {
        match (left_type, right_type) {
            (knodiq_engine::Type::Int, knodiq_engine::Type::Int) => {
                self.codegen_int_op(op, left, right)
            }
            (knodiq_engine::Type::Float, knodiq_engine::Type::Float) => {
                self.codegen_float_op(op, left, right)
            }
            (left_type, right_type) => {
                self.codegen_broadcast_op(
                    |vals, types, slf, _| match types[0] {
                        knodiq_engine::Type::Int => slf.codegen_int_op(op, vals[0], vals[1]),
                        knodiq_engine::Type::Float => slf.codegen_float_op(op, vals[0], vals[1]),
                        _ => panic!("Unsupported type for binary operation: {:?}", types[0]),
                    },
                    vec![left, right],
                    vec![left_type, right_type],
                    module,
                )
                .0
            }
        }
    }

    /// Performs an integer operation based on the operator.
    pub fn codegen_int_op(
        &mut self,
        op: &Operator,
        left: ir::Value,
        right: ir::Value,
    ) -> ir::Value {
        match op {
            Operator::Add => self.builder.ins().iadd(left, right),
            Operator::Subtract => self.builder.ins().isub(left, right),
            Operator::Multiply => self.builder.ins().imul(left, right),
            Operator::Divide => self.builder.ins().udiv(left, right),
            Operator::Modulo => self.builder.ins().urem(left, right),
        }
    }

    /// Performs a float operation based on the operator.
    pub fn codegen_float_op(
        &mut self,
        op: &Operator,
        left: ir::Value,
        right: ir::Value,
    ) -> ir::Value {
        match op {
            Operator::Add => self.builder.ins().fadd(left, right),
            Operator::Subtract => self.builder.ins().fsub(left, right),
            Operator::Multiply => self.builder.ins().fmul(left, right),
            Operator::Divide => self.builder.ins().fdiv(left, right),
            Operator::Modulo => self.call_function("mod", vec![left, right])[0],
        }
    }

    pub fn codegen_broadcast_op<F>(
        &mut self,
        mut op: F,
        vals: Vec<ir::Value>,
        val_types: Vec<knodiq_engine::Type>,
        module: &JITModule,
    ) -> (ir::Value, knodiq_engine::Type)
    where
        F: FnMut(
            Vec<ir::Value>,
            Vec<knodiq_engine::Type>,
            &mut Translator,
            &JITModule,
        ) -> ir::Value,
    {
        let has_array = val_types
            .iter()
            .any(|ty| matches!(ty, knodiq_engine::Type::Array(_)));

        if !has_array {
            let return_type = val_types[0].clone();
            return (op(vals, val_types, self, module), return_type);
        }

        println!("🎯 Direct memory write array operation!");

        let array_types = val_types
            .iter()
            .filter_map(|ty| match ty {
                knodiq_engine::Type::Array(inner) => Some(inner.clone()),
                _ => None,
            })
            .collect::<Vec<_>>();

        let element_type = array_types[0].clone();
        let element_ir_type = get_ir_type(&element_type, module);

        // 空の2要素配列を作成
        let ptr_type = module.target_config().pointer_type();
        let result_ptr = self.builder.ins().get_stack_pointer(ptr_type);

        // 長さ=2を書き込み
        let two = self.builder.ins().iconst(TYPE_INT, 2);
        self.builder
            .ins()
            .store(ir::MemFlags::new(), two, result_ptr, 0);

        // 1つ目の要素を計算＆書き込み
        let idx0 = self.builder.ins().iconst(TYPE_INT, 0);
        let mut first_elements = Vec::new();
        let mut element_types = Vec::new();

        for (j, val_type) in val_types.iter().enumerate() {
            match val_type {
                knodiq_engine::Type::Array(inner_type) => {
                    let first_elem =
                        self.load_arr_val_at(get_ir_type(inner_type, module), vals[j], idx0);
                    first_elements.push(first_elem);
                    element_types.push(*inner_type.clone());
                }
                _ => {
                    first_elements.push(vals[j]);
                    element_types.push(val_type.clone());
                }
            }
        }

        let result_elem0 = op(first_elements, element_types.clone(), self, module);

        // 1つ目の要素を直接メモリに書き込み
        let offset0 = TYPE_INT.bytes() as i32; // offset 4
        self.builder
            .ins()
            .store(ir::MemFlags::new(), result_elem0, result_ptr, offset0);

        // 2つ目の要素を計算＆書き込み
        let idx1 = self.builder.ins().iconst(TYPE_INT, 1);
        let mut second_elements = Vec::new();

        for (j, val_type) in val_types.iter().enumerate() {
            match val_type {
                knodiq_engine::Type::Array(inner_type) => {
                    let second_elem =
                        self.load_arr_val_at(get_ir_type(inner_type, module), vals[j], idx1);
                    second_elements.push(second_elem);
                }
                _ => {
                    second_elements.push(vals[j]);
                }
            }
        }

        let result_elem1 = op(second_elements, element_types, self, module);

        // 2つ目の要素を直接メモリに書き込み
        let offset1 = TYPE_INT.bytes() as i32 + element_ir_type.bytes() as i32; // offset 8
        self.builder
            .ins()
            .store(ir::MemFlags::new(), result_elem1, result_ptr, offset1);

        (result_ptr, knodiq_engine::Type::Array(element_type))
    }

    pub fn recurse_op_broadcast<F>(
        &mut self,
        op: &mut F,
        args: Vec<ir::Value>,
        arg_types: Vec<knodiq_engine::Type>,
        module: &JITModule,
    ) -> (ir::Value, knodiq_engine::Type)
    where
        F: FnMut(
            Vec<ir::Value>,
            Vec<knodiq_engine::Type>,
            &mut Translator,
            &JITModule,
        ) -> ir::Value,
    {
        let has_array = arg_types
            .iter()
            .any(|ty| matches!(ty, knodiq_engine::Type::Array(_)));

        if !has_array {
            // If there are no arrays, just apply the operation directly
            let return_type = arg_types[0].clone();
            return (op(args, arg_types, self, module), return_type);
        }

        let array_types: Vec<_> = arg_types
            .iter()
            .filter_map(|ty| match ty {
                knodiq_engine::Type::Array(inner) => Some(inner.clone()),
                _ => None,
            })
            .collect();

        if array_types.is_empty() {
            unreachable!("Expected at least one array type!?");
        }

        let element_type = array_types[0].clone();
        let element_ir_type = get_ir_type(&element_type, module);
        let element_size = element_ir_type.bytes() as i64;
        let element_size_ir_val = self.builder.ins().iconst(TYPE_INT, element_size);

        // Calculate the offset for the data part of the array
        // The first `TYPE_INT` bytes store the size of the array
        let data_offset = TYPE_INT.bytes() as i64;
        let data_offset_ir_val = self.builder.ins().iconst(TYPE_INT, data_offset);

        // TODO: Check if all arrays have the same size
        let array_size = self.load_arr_size(args[0]);

        // Create a new array to hold the results
        let result_var = self.new_var();
        self.builder
            .declare_var(result_var, module.target_config().pointer_type());

        // Initialize with an empty array
        let empty_array = self.vec_as_array(Vec::new(), element_ir_type, module);
        self.builder.def_var(result_var, empty_array);

        // Iterate over the array elements
        self.codegen_loop(array_size, |slf, idx| {
            // Load the current values from the arrays
            let mut current_vals = Vec::new();
            let mut current_types = Vec::new();

            for (i, arg_type) in arg_types.iter().enumerate() {
                match arg_type {
                    knodiq_engine::Type::Array(inner_type) => {
                        let elem =
                            slf.load_arr_val_at(get_ir_type(inner_type, module), args[i], idx);
                        current_vals.push(elem);
                        current_types.push(*inner_type.clone());
                    }
                    _ => {
                        // If it's not an array, just use the value directly
                        current_vals.push(args[i]);
                        current_types.push(arg_type.clone());
                    }
                }
            }

            // Apply the operation to the current values
            // let result_element = slf
            //     .recurse_op_broadcast(op, current_vals, current_types, module)
            //     .0;
            let result_element = op(current_vals, current_types, slf, module);

            // Store the result in the result array
            let result_array_addr = slf.builder.use_var(result_var);
            let result_elem_offset = slf.builder.ins().imul(idx, element_size_ir_val);
            // Skip the first `TYPE_INT` bytes which store the size of the array
            let result_data_start = slf
                .builder
                .ins()
                .iadd(result_array_addr, data_offset_ir_val);
            let result_elem_addr = slf
                .builder
                .ins()
                .iadd(result_data_start, result_elem_offset);

            // Store the result element at the calculated address
            slf.builder
                .ins()
                .store(ir::MemFlags::new(), result_element, result_elem_addr, 0);
        });

        let final_result = self.builder.use_var(result_var);
        (final_result, knodiq_engine::Type::Array(element_type))
    }

    /// Converts a vector of IR values into an array pointer.
    /// This function assumes that the values are of the same type.
    ///
    /// # Array type definition:
    /// ## Value part
    /// - `ptr: pointer` The pointer to the array data.
    ///
    /// ## Data part
    /// - `size: int` The size of the array (length).
    /// - `[data: pointer]` Content of the array which is the pointer to the each data.
    pub fn vec_as_array(
        &mut self,
        elems: Vec<ir::Value>,
        elem_type: ir::Type,
        module: &JITModule,
    ) -> ir::Value {
        let ptr_type = module.target_config().pointer_type();

        // 必要なサイズを計算
        let header_size = TYPE_INT.bytes() as usize;
        let elem_size = elem_type.bytes() as usize;
        let total_size = header_size + (elems.len() * elem_size);

        // スタックポインタを取得し、明示的にサイズを確保
        let arr_ptr = self.builder.ins().get_stack_pointer(ptr_type);

        // 配列長を設定
        let len_val = self.builder.ins().iconst(TYPE_INT, elems.len() as i64);
        self.builder
            .ins()
            .store(ir::MemFlags::new(), len_val, arr_ptr, 0);

        // 各要素を設定
        for (i, elem) in elems.iter().enumerate() {
            let offset = header_size as i32 + (i * elem_size) as i32;
            self.builder
                .ins()
                .store(ir::MemFlags::new(), *elem, arr_ptr, offset);
        }

        // 未使用部分をゼロクリア（安全のため）
        let used_size = header_size + (elems.len() * elem_size);
        if total_size > used_size {
            let zero = self.builder.ins().iconst(ir::types::I8, 0);
            for i in used_size..total_size {
                self.builder
                    .ins()
                    .store(ir::MemFlags::new(), zero, arr_ptr, i as i32);
            }
        }

        arr_ptr
    }

    /// Iterates over an array at the given address, jumping to the provided block for each element.
    pub fn iter_arr<F>(&mut self, addr: ir::Value, val_type: ir::Type, iter_fn: F)
    where
        F: FnOnce(&mut Self, ir::Value, ir::Type),
    {
        let current_block = self.builder.current_block().unwrap();

        let data_bytes = self.builder.ins().iconst(val_type, val_type.bytes() as i64);

        // Get the size of the array
        let size = self
            .builder
            .ins()
            .load(TYPE_INT, ir::MemFlags::new(), addr, 0);

        // Calculate the starting address of the data by adding the size of the size
        let data_offset = self.builder.ins().iconst(TYPE_INT, TYPE_INT.bytes() as i64);

        // LOOP
        let loop_block = self.builder.create_block();
        let next_block = self.builder.create_block();
        self.builder.append_block_param(loop_block, TYPE_INT);

        self.builder
            .ins()
            .jump(loop_block, &[BlockArg::Value(data_offset)]);
        self.builder.switch_to_block(loop_block);

        self.builder.insert_block_after(loop_block, current_block);
        self.builder.insert_block_after(next_block, current_block);

        // --- LOOP BODY ---
        let i = self.builder.block_params(loop_block)[0];

        // Calculate the address of the current elementl
        let offset_bytes = self.builder.ins().imul(i, data_bytes);
        let data_addr = self.builder.ins().iadd(addr, offset_bytes);
        let target_addr = self.builder.ins().iadd(data_addr, data_offset);

        // Load the value from the array
        let current_val = self
            .builder
            .ins()
            .load(val_type, ir::MemFlags::new(), target_addr, 0);
        iter_fn(self, current_val, val_type);

        // --- LOOP CONDITION ---
        let cmp = self
            .builder
            .ins()
            .icmp(ir::condcodes::IntCC::SignedLessThan, i, size);
        let next_i = self.builder.ins().iadd(i, ir::Value::from_u32(1));
        self.builder
            .ins()
            .brif(cmp, loop_block, &[BlockArg::Value(next_i)], next_block, []);

        self.builder.switch_to_block(next_block);
    }

    /// Loads an array value at a specific index from the given address.
    pub fn load_arr_val_at(
        &mut self,
        val_type: ir::Type,
        addr: ir::Value,
        index: ir::Value,
    ) -> ir::Value {
        // First 4 bytes of the array store the size of the array
        let size_offset = TYPE_INT.bytes() as i64;
        let size_offset_ir_val = self.builder.ins().iconst(TYPE_INT, size_offset);

        // Get the bytes of element in the array
        let type_bytes = self.builder.ins().iconst(TYPE_INT, val_type.bytes() as i64);

        // Calculate the address of the element at the given index
        let offset_bytes = self.builder.ins().imul(index, type_bytes);
        let data_offset = self.builder.ins().iadd(size_offset_ir_val, offset_bytes);
        let target_addr = self.builder.ins().iadd(addr, data_offset);
        let current_val = self
            .builder
            .ins()
            .load(val_type, ir::MemFlags::new(), target_addr, 0);

        current_val
    }

    pub fn load_arr_size(&mut self, addr: ir::Value) -> ir::Value {
        self.builder
            .ins()
            .load(TYPE_INT, ir::MemFlags::new(), addr, 0)
    }

    /// Generates code for a loop that iterates `count` times, executing the body for each iteration.
    pub fn codegen_loop<F>(&mut self, count: ir::Value, mut body: F)
    where
        F: FnMut(&mut Self, ir::Value),
    {
        let current_block = self.builder.current_block().unwrap();

        // Define blocks
        let cond_block = self.builder.create_block();
        let loop_block = self.builder.create_block();
        let next_block = self.builder.create_block();
        self.builder.append_block_param(cond_block, TYPE_INT);
        self.builder.insert_block_after(loop_block, current_block);
        self.builder.insert_block_after(next_block, current_block);

        // Start the loop
        let start_i = ir::Value::from_u32(0);
        self.builder
            .ins()
            .jump(cond_block, [&BlockArg::Value(start_i)]);

        // Loop condition
        self.builder.switch_to_block(cond_block);
        let i = self.builder.block_params(cond_block)[0];
        let cmp = self
            .builder
            .ins()
            .icmp(ir::condcodes::IntCC::SignedLessThan, i, count);
        let next_i = self.builder.ins().iadd(i, ir::Value::from_u32(1));
        self.builder
            .ins()
            .brif(cmp, loop_block, &[], next_block, []);

        // LOOP BODY
        self.builder.switch_to_block(loop_block);
        body(self, i);

        // Jump to the next iteration
        self.builder
            .ins()
            .jump(cond_block, &[BlockArg::Value(next_i)]);

        self.builder.switch_to_block(next_block);
    }

    /// Generates code for a if statement with a condition, a then block, and an else block.
    pub fn codegen_if<ThenFn, ElseFn>(
        &mut self,
        condition: ir::Value,
        mut then_fn: ThenFn,
        mut else_fn: ElseFn,
    ) where
        ThenFn: FnMut(&mut Self) -> Vec<(ir::Value, ir::Type)>,
        ElseFn: FnMut(&mut Self) -> Vec<(ir::Value, ir::Type)>,
    {
        let current_block = self.builder.current_block().unwrap();

        let result_var = self.new_var();

        self.builder.declare_var(result_var, TYPE_INT);

        let then_block = self.builder.create_block();
        let else_block = self.builder.create_block();
        let next_block = self.builder.create_block();

        self.builder.seal_block(current_block);

        self.builder
            .ins()
            .brif(condition, then_block, &[], else_block, &[]);

        // --- THEN BLOCK ---
        self.builder.switch_to_block(then_block);
        let then_res = then_fn(self);
        for (_, res_type) in &then_res {
            self.builder.append_block_param(next_block, *res_type);
        }
        self.builder.ins().jump(
            next_block,
            &then_res
                .iter()
                .map(|(v, _)| BlockArg::Value(*v))
                .collect::<Vec<_>>(),
        );

        // --- ELSE BLOCK ---
        self.builder.switch_to_block(else_block);
        let else_res = else_fn(self);
        for (_, res_type) in &else_res {
            self.builder.append_block_param(next_block, *res_type);
        }
        self.builder.ins().jump(
            next_block,
            &else_res
                .iter()
                .map(|(v, _)| BlockArg::Value(*v))
                .collect::<Vec<_>>(),
        );

        self.builder.switch_to_block(next_block);
    }

    pub fn new_var(&mut self) -> Variable {
        let var = Variable::new(self.var_counter);
        self.var_counter += 1;
        var
    }
}

/// Converts a `knodiq_engine::Type` to an IR type.
pub fn get_ir_type(value_type: &knodiq_engine::Type, module: &JITModule) -> types::Type {
    match value_type {
        knodiq_engine::Type::Int => TYPE_INT,
        knodiq_engine::Type::Float => TYPE_FLOAT,
        knodiq_engine::Type::Array(_) => module.target_config().pointer_type(),
        knodiq_engine::Type::None => types::INVALID,
    }
}
