#![allow(non_snake_case)]

const K: [u32; 64] = [
    0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5,
    0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174,
    0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
    0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967,
    0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85,
    0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
    0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
    0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2,
];

fn rotate_right(x:  u32, n: u32) -> u32 {
    (x >> n) | (x << (32 - n))
}

fn sha256_padding(message: &[u8]) -> Vec<u8> {
    let mut padded = Vec::from(message);
    padded.push(0x80);

    while (padded.len() * 8) % 512 != 448 {
        padded.push(0);
    }

    let bit_len = (message.len() as u64) * 8;
    padded.extend_from_slice(&bit_len.to_be_bytes());
    padded
}

fn sha256(message: &[u8]) -> [u8; 32] {
    let padded_message = sha256_padding(message);
    let mut H: [u32; 8] = [
        0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a,
        0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19,
    ];

    for chunk in padded_message.chunks(64) {
        let mut W = [0u32; 64];
        for i in 0..16 {
            W[i] = u32::from_be_bytes([chunk[4 * i], chunk[4 * i + 1], chunk[4 * i + 2], chunk[4 * i + 3]]);
        }

        for i in 16..64 {
            let s0 = rotate_right(W[i - 15], 7) ^ rotate_right(W[i - 15], 18) ^ (W[i - 15] >> 3);
            let s1 = rotate_right(W[i - 2], 17) ^ rotate_right(W[i - 2], 19) ^ (W[i - 2] >> 10);
            W[i] = W[i -16].wrapping_add(s0).wrapping_add(W[i - 7]).wrapping_add(s1);
        }

        let mut a = H[0];
        let mut b = H[1];
        let mut c = H[2];
        let mut d = H[3];
        let mut e = H[4];
        let mut f = H[5];
        let mut g = H[6];
        let mut h = H[7];

        for i in 0..64 {
            let s1 = rotate_right(e, 6) ^ rotate_right(e, 11) ^ rotate_right(e, 25);
            let ch = (e & f) ^ (!e & g);
            let temp1 = h.wrapping_add(s1).wrapping_add(ch).wrapping_add(K[i]).wrapping_add(W[i]);
            let S0 = rotate_right(a, 2) ^ rotate_right(a, 13) ^ rotate_right(a, 22);
            let maj = (a & b) ^ (a & c) ^ (b & c);
            let temp2 = S0.wrapping_add(maj);

            h = g;
            g = f;
            f = e;
            e = d.wrapping_add(temp1);
            d = c;
            c = b;
            b = a;
            a = temp1.wrapping_add(temp2);
        }

        H[0] = H[0].wrapping_add(a);
        H[1] = H[1].wrapping_add(b);
        H[2] = H[2].wrapping_add(c);
        H[3] = H[3].wrapping_add(d);
        H[4] = H[4].wrapping_add(e);
        H[5] = H[5].wrapping_add(f);
        H[6] = H[6].wrapping_add(g);
        H[7] = H[7].wrapping_add(h);
    }

    let mut hash = [0u8; 32];
    for (i, word) in H.iter().enumerate() {
        hash[4 * i..4 * i +  4].copy_from_slice(&word.to_be_bytes());
    }
    hash
}

fn main() {
    let input = b"Hi there";
    let hash = sha256(input);
    for byte in &hash {
        print!("{:02x}", byte);
    }
    println!();
}
