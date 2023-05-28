extern crate crypto;

use crypto::hash::sha256;

fn main() {
    let x = sha256::Input::new("hello world");

    println!("{:x}", x.sha256());
}

#[cfg(test)]
mod tests {
    use super::*;
    use ethnum::U256;

    #[test]
    fn test_sha256() {
        let tests = vec![
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

        for (input, expected) in tests {
            let x = sha256::Input::new(input);
            let output = x.sha256();

            println!("{:x}", output);
            assert_eq!(
                output,
                U256::from_str_hex(expected).unwrap(),
                "input: {}",
                input
            );
        }
    }
}
