use std::iter::FromIterator;

fn encode(parity_bits: usize, data: &[u8]) -> Vec<u8> {
    let n = data.len() + parity_bits;
    assert_eq!(2i32.pow(parity_bits as u32), (n + 1) as i32);
    let mut code: Vec<u8> = Vec::with_capacity(n);
    let mut pi = 0;
    let mut di = 0;
    for i in 1..=n {
        if i as i32 == 2i32.pow(pi as u32) {
            pi += 1;
            code.push(0);
        } else {
            code.push(data[di]);
            di += 1;
        }
    }
    let mut bit_xor = 0;
    for x in 0..n {
        if code[x] == 1 {
            bit_xor ^= x + 1;
        }
    }
    for (i, b) in format!("{:b}", bit_xor).chars().rev().take(n).enumerate() {
        if b == '1' {
            code[2usize.pow((i) as u32) - 1] = 1;
        }
    }
    code
}

fn decode(code: &[u8]) -> (usize, Vec<u8>) {
    let mut code: Vec<u8> = Vec::from_iter(code.iter().cloned());
    let mut error = 0usize;
    for (i, c) in code.iter().enumerate() {
        if *c == 1u8 {
            error ^= i + 1
        }
    }
    // fix error
    println!("error {} {:?}", error, code);
    if error > 0 {
        code[error - 1] ^= 1;
    }
    let mut data: Vec<u8> = Vec::<u8>::new();
    let mut pi = 0;
    for i in 0..code.len() {
        if i as i32 == 2i32.pow(pi as u32) {
            pi += 1;
        } else {
            data.push(code[i]);
        }
    }
    (error - 1, data)
}

fn main() {
    let parity_bits = 3;
    let data: [u8; 4] = [0, 1, 1, 1];
    // generate code
    let mut code = encode(parity_bits, &data);
    println!("hamming code: {:?} -> {:?}", data, code);

    // make error
    code[3] ^= 1;

    // reconstruct
    let (error, recon) = decode(&code);
    println!("error @ {} -> {:?}", error, recon);
}
