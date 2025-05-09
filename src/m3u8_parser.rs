use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

#[derive(Debug)]
pub struct M3u8Data {
    pub ts_segments: Vec<String>,
    pub codec_info: Vec<String>,
}

pub struct M3u8Parser {
    stream_inf_re: Regex,
}

impl M3u8Parser {
    pub fn new() -> Self {
        let stream_inf_re = Regex::new(r#"CODECS="([^"]*)""#)
            .expect("Failed to compile regex for #EXT-X-STREAM-INF");
        M3u8Parser { stream_inf_re }
    }

    pub fn parse(&self, file_path: &Path) -> Result<M3u8Data, io::Error> {
        let file: File = File::open(file_path)?;
        let reader: BufReader<File> = BufReader::new(file);

        let mut ts_segments: Vec<String> = Vec::new();
        let mut codec_info: Vec<String> = Vec::new();

        for line_result in reader.lines() {
            let line: String = line_result?.trim().to_owned();

            if line.is_empty() {
                continue;
            }

            if line.starts_with("#EXT-X-STREAM-INF:") {
                if let Some(caps) = self.stream_inf_re.captures(&line) {
                    if let Some(codecs) = caps.get(1) {
                        codec_info.push(codecs.as_str().to_string());
                    }
                }
            } else if !line.starts_with('#') {
                ts_segments.push(line);
            }
        }

        Ok(M3u8Data {
            ts_segments,
            codec_info,
        })
    }
}
