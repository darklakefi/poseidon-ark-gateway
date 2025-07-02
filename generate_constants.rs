use ark_bn254::Fr;
use ark_serialize::CanonicalSerialize;
use ark_std::{str::FromStr, vec::Vec};
use std::fs::File;
use std::io::Write;

// Include your constants module
mod src {
    pub mod constants;
}

#[derive(CanonicalSerialize)]
pub struct Constants {
    pub c: Vec<Vec<Fr>>,
    pub m: Vec<Vec<Vec<Fr>>>,
    pub n_rounds_f: usize,
    pub n_rounds_p: Vec<usize>,
}

fn main() {
    println!("Generating pre-serialized constants...");
    
    // Load string constants
    let (c_str, m_str) = src::constants::constants();
    
    // Convert to Fr elements
    let mut c: Vec<Vec<Fr>> = Vec::new();
    for i in 0..c_str.len() {
        let mut cci: Vec<Fr> = Vec::new();
        for j in 0..c_str[i].len() {
            let fr: Fr = Fr::from_str(c_str[i][j]).unwrap();
            cci.push(fr);
        }
        c.push(cci);
    }
    
    let mut m: Vec<Vec<Vec<Fr>>> = Vec::new();
    for i in 0..m_str.len() {
        let mut mi: Vec<Vec<Fr>> = Vec::new();
        for j in 0..m_str[i].len() {
            let mut mij: Vec<Fr> = Vec::new();
            for k in 0..m_str[i][j].len() {
                let fr: Fr = Fr::from_str(m_str[i][j][k]).unwrap();
                mij.push(fr);
            }
            mi.push(mij);
        }
        m.push(mi);
    }
    
    // Create the complete Constants struct
    let constants = Constants {
        c,
        m,
        n_rounds_f: 8,
        n_rounds_p: vec![
            56, 57, 56, 60, 60, 63, 64, 63, 60, 66, 60, 65, 70, 60, 64, 68,
        ],
    };
    
    // Serialize the entire struct
    let mut serialized = Vec::new();
    constants.serialize_uncompressed(&mut serialized).unwrap();
    
    // Write to data directory
    std::fs::create_dir_all("data").unwrap();
    let mut file = File::create("data/constants.bin").unwrap();
    file.write_all(&serialized).unwrap();
    
    println!("Generated data/constants.bin ({} bytes)", serialized.len());
    println!("Constants generation complete!");
} 