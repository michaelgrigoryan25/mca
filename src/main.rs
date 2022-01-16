use std::fs;
use std::io::{BufRead, Read, Write};

#[cfg(unix)]
const NEWLINE: &str = "\n";
#[cfg(windows)]
const NEWLINE: &str = "\r\n";

struct CompressResult {
    pub compressed: String,
    pub index: Vec<String>,
}

fn compress(content: &str) -> CompressResult {
    let mut shared_index: Vec<&str> = vec![];
    let mut compressed: Vec<Vec<String>> = vec![];
    let lines = content.split(NEWLINE).collect::<Vec<&str>>();

    for line in lines.iter() {
        if !line.trim().is_empty() {
            // Vector with the compressed values from current line
            let mut compressed_line: Vec<String> = vec![];

            for chunk in line.split_whitespace() {
                // Checking how many times this chunk appears in the text
                // by looping through each line and each word split by
                // whitespace. If there is only one occurrence of the chunk
                // then a new index will not be pushed to the shared index.
                let mut occurrences: usize = 0;
                lines.iter().for_each(|l| {
                    l.split_whitespace().for_each(|c| {
                        if c.contains(chunk) {
                            occurrences += 1
                        }
                    });
                });

                if occurrences == 1 {
                    compressed_line.push(chunk.to_string());
                } else if shared_index.contains(&chunk) {
                    let position = shared_index.iter().position(|it| it == &chunk).unwrap();
                    compressed_line.push(format!("{}", position).to_string());
                } else {
                    shared_index.push(chunk);
                    compressed_line.push(format!("{}", shared_index.len() - 1).to_string())
                }
            }

            compressed.push(compressed_line);
        }
    }

    CompressResult {
        index: shared_index
            .iter()
            .map(|it| it.to_string())
            .collect::<Vec<String>>(),

        compressed: compressed
            .iter()
            .map(|line| line.join(" "))
            .collect::<Vec<String>>()
            .join(NEWLINE),
    }
}

struct DecompressResult {
    pub decompressed: String,
}

fn decompress(content: &str) -> DecompressResult {
    let mut segments = content.split(NEWLINE);
    let index = segments.nth(0);
    let text = segments.collect::<Vec<&str>>().join(NEWLINE);

    if let Some(index) = index {
        // For storing decompressed line vectors
        let mut decompressed: Vec<Vec<String>> = vec![];
        // Parsing the shared index from the first line of the compressed file.
        let shared_index = serde_json::from_str::<Vec<String>>(index.trim()).unwrap();

        for line in text.split(NEWLINE) {
            if !line.trim().is_empty() {
                let mut decompressed_line: Vec<String> = vec![];

                for chunk in line.split_whitespace().collect::<Vec<&str>>() {
                    if let Ok(shared_index_position) = chunk.parse::<usize>() {
                        if let Some(indice) = shared_index.get(shared_index_position) {
                            decompressed_line.push(indice.to_string());
                        } else {
                            panic!("corrupted file cannot be decompressed.");
                        }
                    } else {
                        decompressed_line.push(chunk.to_string());
                    }
                }

                decompressed.push(decompressed_line);
            }
        }

        DecompressResult {
            decompressed: decompressed
                .iter()
                .map(|line| line.join(" "))
                .collect::<Vec<String>>()
                .join(NEWLINE),
        }
    } else {
        panic!("corrupted file cannot be decompressed.")
    }
}

fn main() -> std::io::Result<()> {
    let mut input = String::new();
    loop {
        println!("select an option: (c)ompress/(d)ecompress:");
        std::io::stdin().read_line(&mut input)?;
        match input.trim().to_lowercase().as_str() {
            "d" | "c" => break,
            _v => {
                eprint!("invalid option: `{_v}`. try again.");
                input.clear();
                continue;
            }
        }
    }

    println!("enter file path:");
    let mut path = String::new();
    std::io::stdin().lock().read_line(&mut path)?;

    let mut content = String::new();
    fs::File::open(path.trim())?.read_to_string(&mut content)?;

    if input.trim() == "c" {
        let result = compress(&content);
        let formatted = format!(
            "{}{}{}",
            serde_json::to_string(&result.index).unwrap(),
            NEWLINE,
            result.compressed
        );
        fs::File::create("compressed.mca")?.write_all(formatted.as_bytes())?;
    } else {
        let result = decompress(&content);
        fs::File::create("decompressed.txt")?.write_all(result.decompressed.as_bytes())?;
    }

    Ok(())
}
