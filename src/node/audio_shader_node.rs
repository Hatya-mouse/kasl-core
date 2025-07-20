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

use std::{collections::HashMap, error::Error};

use crate::{
    Compiler, Parser, Program, SemanticAnalyzer, SymbolInfo, SyntaxError, compile, run_fn,
};
use knodiq_engine::{Node, NodeId, Value, error::TrackError};

pub struct AudioShaderNode {
    id: NodeId,
    name: String,
    pub input: HashMap<String, SymbolInfo>,
    pub output: HashMap<String, SymbolInfo>,
    pub shader: String,
    pub program: Option<Program>,
}

impl AudioShaderNode {
    /// Creates a new AudioShaderNode instance.
    pub fn new() -> Self {
        AudioShaderNode {
            id: NodeId::new_v4(),
            name: "Audio Shader Node".to_string(),
            input: HashMap::new(),
            output: HashMap::new(),
            shader: "".to_string(),
            program: None,
        }
    }

    /// Sets the shader code for the node.
    pub fn set_shader(&mut self, shader: String) -> Result<(), Box<dyn Error>> {
        self.shader = shader;

        // Compile the shader code into a program.
        let parser = Parser::new();
        let program = match parser.parse(&self.shader) {
            Ok(program) => program,
            Err(_) => return Err(Box::new(SyntaxError::new())),
        };

        // Check for errors in the program.
        let mut analyzer = SemanticAnalyzer::new();
        match analyzer.analyze(&program) {
            Ok(program) => {
                self.program = Some(program);
                self.input = analyzer.get_input_table();
                self.output = analyzer.get_output_table();
                return Ok(());
            }
            Err(error) => return Err(Box::new(error)),
        }
    }

    /// Gets the shader code of the node.
    pub fn get_shader(&self) -> &str {
        &self.shader
    }
}

impl Node for AudioShaderNode {
    fn process(
        &mut self,
        _sample_rate: usize,
        _samples_per_beat: f32,
        _channels: usize,
        _chunk_start: usize,
        _chunk_end: usize,
        _track_id: u32,
    ) -> Result<(), Box<dyn TrackError>> {
        // let program = match self.program.as_ref() {
        //     Some(program) => program,
        //     None => return Ok(()),
        // };

        // let mut interpreter = Interpreter::new(
        //     program.clone(),
        //     sample_rate,
        //     samples_per_beat,
        //     channels,
        //     chunk_start,
        //     chunk_end,
        // );

        // let output_table = interpreter
        //     .execute(self.input.clone())
        //     .map_err(|e| Box::new(e) as Box<dyn TrackError>)?;
        // self.output = output_table;

        let mut input_vec = Vec::new();
        for (_, symbol_info) in &self.input {
            input_vec.push(match &symbol_info.value {
                Some(v) => v.clone(),
                None => return Ok(()),
            });
        }

        let mut compiler = match Compiler::new() {
            Ok(c) => c,
            Err(_) => return Ok(()),
        };

        let mut exec = match compile(&mut compiler, &self.shader) {
            Ok(e) => e,
            Err(_) => return Ok(()),
        };

        let output = match run_fn(&mut exec, input_vec) {
            Ok(r) => r,
            Err(_) => return Ok(()),
        };

        let output_entries: Vec<(String, SymbolInfo)> = self
            .output
            .iter()
            .map(|(key, symbol_info)| (key.clone(), symbol_info.clone()))
            .collect();
        for (i, (key, symbol_info)) in output_entries.iter().enumerate() {
            if let Some(value) = output.get(i) {
                self.output.insert(
                    key.clone(),
                    SymbolInfo {
                        value: Some(value.clone()),
                        ..symbol_info.clone()
                    },
                );
            } else {
                self.output.insert(key.clone(), symbol_info.clone());
            }
        }

        Ok(())
    }

    fn get_input(&self, key: &str) -> Option<Value> {
        self.input.get(key).and_then(|info| info.value.clone())
    }

    fn set_input(&mut self, key: &str, value: Value) {
        if self.input.contains_key(key) {
            self.input.get_mut(key).unwrap().value = Some(value);
        }
    }

    fn get_input_list(&self) -> Vec<String> {
        self.input.keys().cloned().collect()
    }

    fn get_output(&self, key: &str) -> Option<Value> {
        self.output.get(key).and_then(|info| info.value.clone())
    }

    fn get_output_list(&self) -> Vec<String> {
        self.output.keys().cloned().collect()
    }

    fn get_type(&self) -> String {
        "AudioShaderNode".to_string()
    }

    fn set_id(&mut self, id: NodeId) {
        self.id = id;
    }

    fn get_id(&self) -> NodeId {
        self.id.clone()
    }

    fn set_name(&mut self, name: String) {
        self.name = name;
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn is_input(&self) -> bool {
        false
    }

    fn is_output(&self) -> bool {
        false
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

impl Clone for AudioShaderNode {
    fn clone(&self) -> Self {
        AudioShaderNode {
            id: self.id.clone(),
            name: self.name.clone(),
            input: self.input.clone(),
            output: self.output.clone(),
            shader: self.shader.clone(),
            program: self.program.clone(),
        }
    }
}
