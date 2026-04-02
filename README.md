# KASL

KASL (Knodiq Audio Shading Language) is a programming language designed for generating or processing audio signals in Knodiq.
Available under the Apache-2.0 License.

# The `kasl` crate

This crate is an umbrella crate which includes re-exports to `kasl-core`, `kasl-ir` and `kasl-cranelift-backend` crate.

# To Get Started

If you are considering to run KASL program, you may want to use my [`kaslc`](https://github.com/hatya-mouse/kaslc) command line tool, which can run KASL programs from your terminal.

If you want to use KASL in your own project, you should use this `kasl` crate, which provides a Rust API for running KASL programs.
In this case, you can use `core::KaslCompiler` to easily compile and run KASL programs from your Rust code.
