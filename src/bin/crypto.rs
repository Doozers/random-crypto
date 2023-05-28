extern crate crypto;

use crypto::hash::sha;
use crypto::hash::sha::Variant;

fn main() {
    let x = sha::Input::new("", Variant::SHA256);

    println!("{:0x}", x.digest());
}

#[cfg(test)]
mod tests {
    use super::*;
    use ethnum::U256;

    #[test]
    fn test_sha256() {
        let tests256 = vec![
            (
                "",
                "0xe3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
            ),
            (
                "The quick brown fox jumps over the lazy dog",
                "0xd7a8fbb307d7809469ca9abcb0082e4f8d5651e46d3cdb762d02d0bf37c9e592",
            ),
            (
                "The quick brown fox jumps over the lazy dog.",
                "0xef537f25c895bfa782526529a9b63d97aa631564d5d789c2b765448c8635fb6c",
            ),
            (
                "Sed ut perspiciatis unde omnis iste natus error sit voluptatem accusantium \
            doloremque laudantium, totam rem aperiam, eaque ipsa quae ab illo inventore \
            veritatis et quasi architecto beatae vitae dicta sunt explicabo. Nemo enim ipsam \
            voluptatem quia voluptas sit aspernatur aut odit aut fugit, sed quia consequuntur \
            magni dolores eos qui ratione voluptatem sequi nesciunt. Neque porro quisquam est, \
            qui dolorem ipsum quia dolor sit amet, consectetur, adipisci velit, sed quia non \
            numquam eius modi tempora incidunt ut labore et dolore magnam aliquam quaerat \
            voluptatem. Ut enim ad minima veniam, quis nostrum exercitationem ullam corporis \
            suscipit laboriosam, nisi ut aliquid ex ea commodi consequatur? Quis autem vel eum \
            iure reprehenderit qui in ea voluptate velit esse quam nihil molestiae consequatur, \
            vel illum qui dolorem eum fugiat quo voluptas nulla pariatur?",
                "0x5d84c72255e0fcf3448125b2832846993bc7604cf352ad0ff714072a8c681d92",
            ),
        ];

        for (input, expected) in tests256 {
            let x = sha::Input::new(input, Variant::SHA256);
            let output = x.digest();

            println!("{:x}", output);
            assert_eq!(
                output,
                U256::from_str_hex(expected).unwrap(),
                "input: {}",
                input
            );
        }
    }

    #[test]
    fn test_sha224() {
        let tests224 = vec![
            (
                "",
                "0xd14a028c2a3a2bc9476102bb288234c415a2b01f828ea62ac5b3e42f",
            ),
            (
                "The quick brown fox jumps over the lazy dog",
                "0x730e109bd7a8a32b1cb9d9a09aa2325d2430587ddbc0c38bad911525",
            ),
            (
                "The quick brown fox jumps over the lazy dog.",
                "0x619cba8e8e05826e9b8c519c0a5c68f4fb653e8a3d8aa04bb2c8cd4c",
            ),
            (
                "Sed ut perspiciatis unde omnis iste natus error sit voluptatem accusantium \
            doloremque laudantium, totam rem aperiam, eaque ipsa quae ab illo inventore \
            veritatis et quasi architecto beatae vitae dicta sunt explicabo. Nemo enim ipsam \
            voluptatem quia voluptas sit aspernatur aut odit aut fugit, sed quia consequuntur \
            magni dolores eos qui ratione voluptatem sequi nesciunt. Neque porro quisquam est, \
            qui dolorem ipsum quia dolor sit amet, consectetur, adipisci velit, sed quia non \
            numquam eius modi tempora incidunt ut labore et dolore magnam aliquam quaerat \
            voluptatem. Ut enim ad minima veniam, quis nostrum exercitationem ullam corporis \
            suscipit laboriosam, nisi ut aliquid ex ea commodi consequatur? Quis autem vel eum \
            iure reprehenderit qui in ea voluptate velit esse quam nihil molestiae consequatur, \
            vel illum qui dolorem eum fugiat quo voluptas nulla pariatur?",
                "0x6c425b94ca846dda0b10f8cad3cc818a5ba6f58ab600e5327306650d",
            ),
        ];

        for (input, expected) in tests224 {
            let x = sha::Input::new(input, Variant::SHA224);
            let output = x.digest();

            assert_eq!(
                output,
                U256::from_str_hex(expected).unwrap(),
                "input: {} => {:x}",
                input,
                output
            );
        }
    }
}
