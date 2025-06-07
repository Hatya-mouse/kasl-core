use knodiq_audio_shader::{Interpreter, Lexer, Parser, SemanticAnalyzer};

fn main() {
    divan::main();
}

#[divan::bench]
fn audio_shader_sample_processing() {
    let input = "
    input float in_buffer = 0.1
    input float gain = 0.8
    output float out_buffer
    out_buffer = in_buffer * gain
    ";
    let lexer = Lexer::new(input.to_string());
    let tokens = lexer.tokenize();
    let parser = Parser::new(tokens);
    let program = parser.parse();
    match &program {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Error parsing input: {}", e);
        }
    }

    let program = program.unwrap();

    let mut semantic_analyzer = SemanticAnalyzer::new();
    semantic_analyzer
        .analyze(&program)
        .map_err(|e| format!("意味解析エラーにゃ: {:?}", e));
    let ui_params_map = semantic_analyzer.input_table;

    let mut ui_parameters = Vec::new();
    for (_, info) in ui_params_map {
        ui_parameters.push(info.clone());
    }

    let mut interpreter = Interpreter::new(program);
    divan::black_box_drop(
        // Profile the execution time
        match interpreter.execute() {
            Ok(_) => {}
            Err(_) => return,
        },
    );
}
