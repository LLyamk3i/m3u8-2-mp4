pub mod m3u8_parser;
pub mod m3u8_validator;
pub mod ffmpeg_remuxer;

use std::path::{Path, PathBuf};
use m3u8_parser::M3u8Parser;
use m3u8_validator::Segments;
use ffmpeg_remuxer::FfmpegRemuxer;

fn main() {
    println!("Attempting to parse M3U8 file...");

    let parser = M3u8Parser::new();

    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("You must provide a path to a M3U8 file as an argument!");
        std::process::exit(1);
    }
    let m3u8_path = Path::new(&args[1]);

    match parser.parse(m3u8_path) {
        Ok(data) => {
            println!("Successfully parsed M3U8 file!");
            println!("TS Segments found: {}", data.ts_segments.len());
            if std::env::var("VERBOSE").is_ok() {
                for segment in &data.ts_segments {
                    println!("  - {}", segment);
                }
            }
            println!("Codec Info found: {}", data.codec_info.len());
            if std::env::var("VERBOSE").is_ok() {
                for codec in &data.codec_info {
                    println!("  - {}", codec);
                }
            }
            let segment_paths: Vec<PathBuf> = data.ts_segments.iter().map(PathBuf::from).collect();
            let validation: Segments = Segments::validate(&segment_paths);

            println!("Valid segments: {}/{}", validation.valid.len(), segment_paths.len());
            if !validation.missing.is_empty() {
                eprintln!("Missing segments: {}", validation.missing.len());
                for missing_segment in &validation.missing {
                    eprintln!("  - {}", missing_segment.display());
                }
            }

            if !validation.valid.is_empty() {
                println!("Attempting to remux valid segments to output.mp4...");
                let remuxer = FfmpegRemuxer::new();
                let output_path = m3u8_path.with_extension("mp4");
                match remuxer.remux(&validation.valid, &output_path) {
                    Ok(_) => println!("Successfully remuxed segments to {}", output_path.display()),
                    Err(e) => eprintln!("Error remuxing segments: {}", e),
                }
            } else {
                println!("No valid segments to remux.");
            }
        }
        Err(e) => {
            eprintln!("Error parsing M3U8 file: {}", e);
        }
    }
}
