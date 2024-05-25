use crate::fuzzy_int::*;
use array_init::array_init;

pub fn fuzzy_sha1(message_slice: &[&FInt8]) -> FInt<160> {
    let mut h = [
        FInt32::from(0x67452301),
        FInt32::from(0xEFCDAB89),
        FInt32::from(0x98BADCFE),
        FInt32::from(0x10325476),
        FInt32::from(0xC3D2E1F0),
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

        let mut w: [&FInt32; 80] = array_init(|_| &FInt32::init());

        for i in 0..16 {
            *w[i] = chunk[i];
        }

        for i in 16..80 {
            w[i] = (w[i - 3] ^ w[i - 8] ^ w[i - 14] ^ w[i - 16]).lrotate(1);
        }

        for i in 0..80 {
            let f;
            let k;
            match i {
                0..=19 => {
                    f = (d[1] & d[2]) | (!d[1] & d[3]);
                    k = FInt32::from(0x5A827999);
                }

                20..=39 => {
                    f = d[1] ^ d[2] ^ d[3];
                    k = FInt32::from(0x6ED9EBA1);
                }

                40..=59 => {
                    f = (d[1] & d[2]) | (d[1] & d[3]) | (d[2] & d[3]);
                    k = FInt32::from(0x8F1BBCDC);
                }

                60..=79 => {
                    f = d[1] ^ d[2] ^ d[3];
                    k = FInt32::from(0xCA62C1D6);
                }

                _ => {
                    panic!();
                }
            }

            let temp = d[0].lrotate(5) + f + d[4] + k + &w[i];
            d[4] = d[3];
            d[3] = d[2];
            d[2] = d[1].lrotate(30);
            d[1] = d[0];
            d[0] = temp;
        }

        for i in 0..5 {
            h[i] = h[i] + d[i];
        }
    }

    FInt::combine(&h)
}

pub fn fuzzy_sha1_str(hash: FInt<160>) -> String {
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
    fn test_fuzzy_sha1() {
        let test_cases = vec![
            ("", "da39a3ee5e6b4b0d3255bfef95601890afd80709"),
            (
                "The quick brown fox jumps over the lazy dog",
                "2fd4e1c67a2d28fced849ee1bb76e7391b93eb12",
            ),
            (
                "The quick brown fox jumps over the lazy cog",
                "de9f2c7fd25e1b3afad3e85a0bd17d9b100db4b3",
            ),
            ("hello world", "2aae6c35c94fcfb415dbe95f408b9ce91ee846ed"),
        ];

        for (input, expected) in test_cases {
            let input_fuzzy = input
                .chars()
                .map(|c| FInt8::from(c as u8 as usize))
                .collect::<Vec<_>>();
            let result = fuzzy_sha1(input_fuzzy.as_slice());
            assert_eq!(fuzzy_sha1_str(result), expected);
        }
    }
}
