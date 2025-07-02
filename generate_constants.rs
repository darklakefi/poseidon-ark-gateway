use ark_bn254::Fr;
use ark_std::{str::FromStr, vec::Vec};
use std::fs::File;
use std::io::Write;

// Include your constants module
mod src {
    pub mod constants;
}

fn main() {
    println!("Generating static constants code...");
    
    // Load string constants
    let (c_str, m_str) = src::constants::constants();
    
    let mut output = String::new();
    output.push_str("use ark_bn254::Fr;\n");
    output.push_str("use ark_ff::MontFp;\n\n");
    
    // Generate C constants
    output.push_str("pub static C_CONSTANTS: &[&[Fr]] = &[\n");
    for i in 0..c_str.len() {
        output.push_str("    &[\n");
        for j in 0..c_str[i].len() {
            output.push_str(&format!("        MontFp!(\"{}\"),\n", c_str[i][j]));
        }
        output.push_str("    ],\n");
    }
    output.push_str("];\n\n");
    
    // Generate M constants
    output.push_str("pub static M_CONSTANTS: &[&[&[Fr]]] = &[\n");
    for i in 0..m_str.len() {
        output.push_str("    &[\n");
        for j in 0..m_str[i].len() {
            output.push_str("        &[\n");
            for k in 0..m_str[i][j].len() {
                output.push_str(&format!("            MontFp!(\"{}\"),\n", m_str[i][j][k]));
            }
            output.push_str("        ],\n");
        }
        output.push_str("    ],\n");
    }
    output.push_str("];\n\n");
    
    // Generate rounds constants
    output.push_str("pub const N_ROUNDS_F: usize = 8;\n");
    output.push_str("pub static N_ROUNDS_P: &[usize] = &[56, 57, 56, 60, 60, 63, 64, 63, 60, 66, 60, 65, 70, 60, 64, 68];\n");
    
    // Write to src directory
    let mut file = File::create("src/static_constants.rs").unwrap();
    file.write_all(output.as_bytes()).unwrap();
    
    println!("Generated src/static_constants.rs");
    println!("Static constants generation complete!");
} 