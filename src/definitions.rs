use std::{fs::File, io::BufReader};
use rodio::Decoder;

pub fn get_source(filename: &str) -> Decoder<BufReader<File>> {
    let source = Decoder::new(
        BufReader::new(
            File::open(format!("{filename}"))
                .expect("failed to open file")
        )
    );
    return source.expect("Failed to decode file")
}
