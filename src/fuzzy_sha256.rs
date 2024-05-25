use crate::fuzzy_int::*;

pub fn fuzzy_sha256(message_slice: &[&FInt8]) -> FInt<256> {
    let k: [&FInt32; 64] = [
        FInt32::from(0x428a2f98),
        FInt32::from(0x71374491),
        FInt32::from(0xb5c0fbcf),
        FInt32::from(0xe9b5dba5),
        FInt32::from(0x3956c25b),
        FInt32::from(0x59f111f1),
        FInt32::from(0x923f82a4),
        FInt32::from(0xab1c5ed5),
        FInt32::from(0xd807aa98),
        FInt32::from(0x12835b01),
        FInt32::from(0x243185be),
        FInt32::from(0x550c7dc3),
        FInt32::from(0x72be5d74),
        FInt32::from(0x80deb1fe),
        FInt32::from(0x9bdc06a7),
        FInt32::from(0xc19bf174),
        FInt32::from(0xe49b69c1),
        FInt32::from(0xefbe4786),
        FInt32::from(0x0fc19dc6),
        FInt32::from(0x240ca1cc),
        FInt32::from(0x2de92c6f),
        FInt32::from(0x4a7484aa),
        FInt32::from(0x5cb0a9dc),
        FInt32::from(0x76f988da),
        FInt32::from(0x983e5152),
        FInt32::from(0xa831c66d),
        FInt32::from(0xb00327c8),
        FInt32::from(0xbf597fc7),
        FInt32::from(0xc6e00bf3),
        FInt32::from(0xd5a79147),
        FInt32::from(0x06ca6351),
        FInt32::from(0x14292967),
        FInt32::from(0x27b70a85),
        FInt32::from(0x2e1b2138),
        FInt32::from(0x4d2c6dfc),
        FInt32::from(0x53380d13),
        FInt32::from(0x650a7354),
        FInt32::from(0x766a0abb),
        FInt32::from(0x81c2c92e),
        FInt32::from(0x92722c85),
        FInt32::from(0xa2bfe8a1),
        FInt32::from(0xa81a664b),
        FInt32::from(0xc24b8b70),
        FInt32::from(0xc76c51a3),
        FInt32::from(0xd192e819),
        FInt32::from(0xd6990624),
        FInt32::from(0xf40e3585),
        FInt32::from(0x106aa070),
        FInt32::from(0x19a4c116),
        FInt32::from(0x1e376c08),
        FInt32::from(0x2748774c),
        FInt32::from(0x34b0bcb5),
        FInt32::from(0x391c0cb3),
        FInt32::from(0x4ed8aa4a),
        FInt32::from(0x5b9cca4f),
        FInt32::from(0x682e6ff3),
        FInt32::from(0x748f82ee),
        FInt32::from(0x78a5636f),
        FInt32::from(0x84c87814),
        FInt32::from(0x8cc70208),
        FInt32::from(0x90befffa),
        FInt32::from(0xa4506ceb),
        FInt32::from(0xbef9a3f7),
        FInt32::from(0xc67178f2),
    ];

    let mut h = [
        FInt32::from(0x6a09e667),
        FInt32::from(0xbb67ae85),
        FInt32::from(0x3c6ef372),
        FInt32::from(0xa54ff53a),
        FInt32::from(0x510e527f),
        FInt32::from(0x9b05688c),
        FInt32::from(0x1f83d9ab),
        FInt32::from(0x5be0cd19),
    ];

    let mut message = message_slice.into_iter().cloned().collect::<Vec<&FInt8>>();
    message.push(FInt8::from(0x80));

    while message.len() % 64 != 56 {
        message.push(FInt8::from(0x00));
    }

    let message_len = message_slice.len() * 8;
    for i in 0..8 {
        message.push(FInt8::from(message_len >> (56 - 8 * i)));
    }

    let words = message
        .chunks(4)
        .map(|chunk| {
            FInt::combine(&[
                chunk.get(3).unwrap_or(&&FInt8::default()),
                chunk.get(2).unwrap_or(&&FInt8::default()),
                chunk.get(1).unwrap_or(&&FInt8::default()),
                chunk.get(0).unwrap_or(&&FInt8::default()),
            ])
        })
        .collect::<Vec<_>>();

    for chunk in words.chunks(16) {
        let mut d = h;

        let mut w: [&FInt32; 80] = array_init::array_init(|_| &FInt32::default());

        for i in 0..16 {
            *w[i] = chunk[i];
        }

        for i in 16..80 {
            w[i] = {
                let p = w[i - 16] + w[i - 7];

                let w0 = w[i - 15];
                let t0 = w0.rrotate(7) ^ w0.rrotate(18) ^ (w0 >> 3);

                let w1 = w[i - 2];
                let t1 = w1.rrotate(17) ^ w1.rrotate(19) ^ (w1 >> 10);

                p + t0 + t1
            };
        }

        for i in 0..64 {
            let a1 = d[4].rrotate(6) ^ d[4].rrotate(11) ^ d[4].rrotate(25);
            let b1 = (d[4] & d[5]) ^ (!d[4] & d[6]);
            let t1 = d[7] + a1 + b1 + k[i] + w[i];

            let a2 = d[0].rrotate(2) ^ d[0].rrotate(13) ^ d[0].rrotate(22);
            let b2 = (d[0] & d[1]) ^ (d[0] & d[2]) ^ (d[1] & d[2]);
            let t2 = a2 + b2;

            d[7] = d[6];
            d[6] = d[5];
            d[5] = d[4];
            d[4] = d[3] + t1;
            d[3] = d[2];
            d[2] = d[1];
            d[1] = d[0];
            d[0] = t1 + t2;
        }

        for i in 0..8 {
            h[i] = h[i] + d[i];
        }
    }

    FInt::combine(&h)
}

pub fn fuzzy_sha256_str(hash: FInt<256>) -> String {
    let mut output = String::new();
    for int in &hash.split::<32>() {
        output.push_str(&format!("{:08x}", int.collapse()));
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fuzzy_sha256() {
        let test_cases = vec![
            (
                "",
                "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
            ),
            (
                "The quick brown fox jumps over the lazy dog",
                "d7a8fbb307d7809469ca9abcb0082e4f8d5651e46d3cdb762d02d0bf37c9e592",
            ),
            (
                "The quick brown fox jumps over the lazy cog",
                "e4c4d8f3bf76b692de791a173e05321150f7a345b46484fe427f6acc7ecc81be",
            ),
            (
                "hello world",
                "b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9",
            ),
        ];

        for (input, expected) in test_cases {
            let input_fuzzy = input
                .chars()
                .map(|c| FInt8::from(c as u8 as usize))
                .collect::<Vec<_>>();
            let result = fuzzy_sha256(&input_fuzzy);
            assert_eq!(fuzzy_sha256_str(result), expected);
        }
    }
}
