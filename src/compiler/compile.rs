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

use inkwell::context::Context;

struct Compiler {}

impl Compiler {
    pub fn setup(inputs: usize, outputs: usize) {
        let context = Context::create();
        let module = context.create_module("shader");
        let builder = context.create_builder();

        // Define types
        let float_type = context.f32_type();
        let i32_type = context.i32_type();
        let ptr_type = float_type.ptr_type(inkwell::AddressSpace::Generic);

        let ndarray_type = context.struct_type(
            &[
                ptr_type.into(),                                          // Pointer to the data
                i32_type.into(),                                          // Dimention of the array
                i32_type.ptr_type(inkwell::AddressSpace::Generic).into(), // Sizes of each dimension
                i32_type.ptr_type(inkwell::AddressSpace::Generic).into(), // Strides of each dimension
            ],
            false,
        );

        // Create function signature
        let op_fn_type = ndarray_type.fn_type(&[ndarray_type; 2], false);
        let add_fn = module.add_function("add", op_fn_type, None);
        let sub_fn = module.add_function("sub", op_fn_type, None);
        let mul_fn = module.add_function("mul", op_fn_type, None);
        let div_fn = module.add_function("div", op_fn_type, None);
        let mod_fn = module.add_function("mod", op_fn_type, None);

        // Declare main function
        let main_fn_type = ndarray_type.fn_type(&[ndarray_type; inputs], false);
        let main_fn = module.add_function("main", main_fn_type, None);

        let entry_basic_block = context.append_basic_block(main_fn, "entry");
        builder.position_at_end(entry_basic_block);
    }
}
