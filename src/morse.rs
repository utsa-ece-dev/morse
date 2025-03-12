use morse_codec::{decoder::Decoder, encoder::Encoder};

/// cbindgen:ignore
const MSG_MAX: usize = 70000;

pub(crate) fn encode_morse(input: &str) -> String {
    let mut encoder = Encoder::<MSG_MAX>::new().with_message(input, true).build();
    encoder.encode_message_all();

    let encoded_charrays = encoder.get_encoded_message_as_morse_charrays();

    let mut result = String::new();

    encoded_charrays.for_each(|charray| {
        if let Some(charray) = charray {
            for ch in charray.iter().filter(|ch| ch.is_some()) {
                result.push(ch.unwrap() as char);
            }
            result.push(' ');
        }
    });

    result.trim_end().to_string()
}

pub(crate) fn decode_morse(input: &str) -> String {
    let mut decoder = Decoder::<MSG_MAX>::new()
        .with_reference_short_ms(100) // assumes 100ms as "short" for simplicity
        .build();

    // This function parses morse symbols and simulates signal events.
    // '.' -> dit (short high)
    // '-' -> dah (long high)
    // ' ' -> inter-character gap (long low)
    let symbols: Vec<&str> = input.split(' ').collect();

    for symbol in symbols {
        if symbol.is_empty() {
            // Treat consecutive spaces as word gaps
            decoder.signal_event(700, false);
            continue;
        }

        for ch in symbol.chars() {
            match ch {
                '.' => {
                    decoder.signal_event(100, true); // dit
                    decoder.signal_event(100, false); // intra-character gap
                }
                '-' => {
                    decoder.signal_event(300, true); // dah
                    decoder.signal_event(100, false); // intra-character gap
                }
                _ => {
                    // Ignore unsupported characters
                }
            }
        }

        // After each letter, add inter-character gap
        decoder.signal_event(300, false);
    }

    decoder.message.as_str().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_can_encode() {
        let morse = encode_morse("SOS sos");
        assert_eq!(morse, "... --- ...");
    }

    #[test]
    fn it_can_decode() {
        let text = "... --- ...";

        let decoded = decode_morse(text);

        assert_eq!(decoded, "SOS");
    }
}
