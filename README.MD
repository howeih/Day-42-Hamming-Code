Day 42: Hamming Code
<br>
Anytime data is transferred or stored, there are errors occurring which leads to data corruption. Computer memory, disk, ethernet, wi-fi, … it happens everywhere. Hamming code was an impressive discovery of how to deal with the problem efficiently.
<br>
To identify errors, we may double each bit. Instead of 1010 we can store 11001100. If a pair of consecutive bits doesn’t match, data is corrupted. That is called parity.
<br>
But doubling is not enough. We can identify there’s an error but we can’t recover. Hence we have to triple the data. Having 111000111000 we can identify corrupted triplets and let the triplet vote for majority to reconstruct the original.
<br>
However, for each bit of data this approach requires additional 2 bits. If we expect an error to occur not more than once out of 255 bits, that’s just wasting.
<br>
Hamming’s idea is the following. For 255 bits we need 8 bits as address space. We can store 247 bits of data and only use 8 bits for parity checks.
<br>
Each parity bit covers positions that have certain bit set to 1 in its address. For example parity bit P1 checks only addresses with mask xxx1, P2 checks only addresses xx1x, P4 checks only addresses x1xx, etc.
<br>
If an error occurs, only parities targeting the corrupted bit are set to 1 and form an address to exact location.
<br>
run:
```
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
```
resut:
```
hamming code: [0, 1, 1, 1] -> [0, 0, 0, 1, 1, 1, 1]
error 4 [0, 0, 0, 0, 1, 1, 1]
error @ 3 -> [0, 1, 1, 1]
```