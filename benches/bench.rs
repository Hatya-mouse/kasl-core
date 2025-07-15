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

use knodiq_audio_shader::{
    Compiler, Interpreter, Parser, SemanticAnalyzer, SymbolInfo, SymbolKind, Value, run,
};
use knodiq_engine::Type;
use std::collections::HashMap;

fn main() {
    divan::main();
}

#[divan::bench]
fn audio_shader_sample_processing() {
    let input = "
    input [[float]] in_buffer
    input float gain = 0.8
    output [[float]] out_buffer
    out_buffer = in_buffer * gain
    ";
    let parser = Parser::new();
    let program = parser.parse(&input);
    match &program {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Error parsing input: {}", e);
        }
    }

    let program = program.unwrap();

    let mut semantic_analyzer = SemanticAnalyzer::new();
    match semantic_analyzer
        .analyze(&program)
        .map_err(|e| format!("{:?}", e))
    {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Semantic analysis error: {}", e);
            return;
        }
    }

    let mut ui_parameters = HashMap::new();
    ui_parameters.insert(
        "in_buffer".to_string(),
        SymbolInfo {
            name: "in_buffer".to_string(),
            kind: SymbolKind::Input,
            value_type: Type::Array(Box::new(Type::Array(Box::new(Type::Float)))),
            value: Some(Value::from_buffer(vec![vec![0.15; 128]; 2])),
        },
    );
    ui_parameters.insert(
        "gain".to_string(),
        semantic_analyzer.input_table.get("gain").cloned().unwrap(),
    );

    let mut interpreter = Interpreter::new(program, 48000, 24000.0, 2, 0, 128);
    divan::black_box_drop(
        // Profile the execution time
        match interpreter.execute(ui_parameters) {
            Ok(_) => {}
            Err(_) => return,
        },
    );
}

#[divan::bench]
fn compare_interpreter_and_jit() {
    let code = "
    input float in_buffer
    output float out_buffer
    output float powered
    var gain = 1.0
    var result = 0.0

    result = in_buffer * gain
    out_buffer = result + 1.25
    powered = pow(in_buffer, 2.0)
    ";

    let parser = Parser::new();
    let program = parser.parse(&code).unwrap();

    let mut analyzer = SemanticAnalyzer::new();
    analyzer.analyze(&program).unwrap();

    let mut interpreter = Interpreter::new(program, 48000, 24000.0, 2, 0, 2);

    let mut input_table = analyzer.input_table.clone();
    input_table.get_mut("in_buffer").unwrap().value =
        Some(Value::from_buffer(vec![vec![2.0, 3.0]; 2]));

    divan::black_box_drop(match interpreter.execute(input_table) {
        Ok(_) => {}
        Err(_) => return,
    });

    let mut compiler = Compiler::new().unwrap();
    divan::black_box_drop(|| run(&mut compiler, &code, vec![Value::Float(2.0)]).unwrap());
}
