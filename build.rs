use ark_bn254::Fr;
use ark_serialize::CanonicalSerialize;
use ark_std::str::FromStr;
use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

#[path = "src/constants.rs"]
mod constants;

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    
    // Load the string constants
    let (c_str, m_str) = constants::constants();
    
    println!("cargo:rerun-if-changed=src/constants.rs");
    
    // Convert c constants to Fr and serialize
    let mut c_bytes = Vec::new();
    
    // First, serialize the length of c
    (c_str.len() as u32).serialize_compressed(&mut c_bytes).unwrap();
    
    for i in 0..c_str.len() {
        // Serialize the length of each inner vector
        (c_str[i].len() as u32).serialize_compressed(&mut c_bytes).unwrap();
        
        for j in 0..c_str[i].len() {
            let fr: Fr = Fr::from_str(c_str[i][j]).unwrap();
            fr.serialize_compressed(&mut c_bytes).unwrap();
        }
    }
    
    // Convert m constants to Fr and serialize
    let mut m_bytes = Vec::new();
    
    // First, serialize the length of m
    (m_str.len() as u32).serialize_compressed(&mut m_bytes).unwrap();
    
    for i in 0..m_str.len() {
        // Serialize the length of each 2D matrix
        (m_str[i].len() as u32).serialize_compressed(&mut m_bytes).unwrap();
        
        for j in 0..m_str[i].len() {
            // Serialize the length of each inner vector
            (m_str[i][j].len() as u32).serialize_compressed(&mut m_bytes).unwrap();
            
            for k in 0..m_str[i][j].len() {
                let fr: Fr = Fr::from_str(m_str[i][j][k]).unwrap();
                fr.serialize_compressed(&mut m_bytes).unwrap();
            }
        }
    }
    
    // Write the serialized constants to files
    let c_path = Path::new(&out_dir).join("constants_c.bin");
    let mut c_file = File::create(&c_path).unwrap();
    c_file.write_all(&c_bytes).unwrap();
    
    let m_path = Path::new(&out_dir).join("constants_m.bin");
    let mut m_file = File::create(&m_path).unwrap();
    m_file.write_all(&m_bytes).unwrap();
    
    println!("Generated constants_c.bin ({} bytes)", c_bytes.len());
    println!("Generated constants_m.bin ({} bytes)", m_bytes.len());
} 