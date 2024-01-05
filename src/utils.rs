use sha3::{Digest, Keccak256};
use std::path::Path;
use tokio::fs::File;
use tokio::io::{self, AsyncBufReadExt, BufReader};

pub async fn read_lines_to_vec<P>(filename: P) -> io::Result<Vec<String>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename).await?;
    let reader = BufReader::new(file);
    let mut lines = Vec::new();
    let mut lines_stream = reader.lines();

    while let Some(line) = lines_stream.next_line().await? {
        lines.push(line);
    }

    Ok(lines)
}

pub fn hasher<T: AsRef<[u8]>>(input: T) -> String {
    let byte_slice = input.as_ref();

    let mut hasher = Keccak256::new();
    hasher.update(byte_slice);
    let result = hasher.finalize();
    let hash_value = format!("{:x}", result);

    hash_value
}
