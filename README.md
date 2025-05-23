# M3U8 Converter (Experimentation)

A Rust-based utility for processing M3U8 playlists and converting their referenced TS segments into a single MP4 file.

## Project Overview

This experimental project provides a robust way to handle HTTP Live Streaming (HLS) content by parsing M3U8 playlist files, validating the availability of the TS segments they reference, and then using FFmpeg to remux those segments into a single MP4 file for easier playback and storage.

The tool is particularly useful for:
- Archiving HLS content for offline viewing
- Converting streaming media to a format that can be played in standard media players
- Processing and validating M3U8 playlists

## Features

- **M3U8 Parsing**: Extracts segment URLs and codec information from M3U8 playlist files
- **Segment Validation**: Verifies the existence of all referenced TS segments before processing
- **MP4 Conversion**: Remuxes the TS segments into a single MP4 file while preserving audio and video streams
- **Codec Support**: Handles H.264 video and AAC audio codecs commonly used in HLS streams
- **Detailed Reporting**: Provides information about missing segments and conversion progress

## Prerequisites

- **Rust**: This project requires Rust (2024 edition) to compile and run. Install from [rust-lang.org](https://www.rust-lang.org/tools/install)
- **FFmpeg**: The `ffmpeg-next` crate requires FFmpeg libraries to be installed on your system
  - Debian/Ubuntu: `sudo apt install ffmpeg libavcodec-dev libavformat-dev libavutil-dev`
  - Other systems: See [FFmpeg download page](https://ffmpeg.org/download.html)

## Usage

### Basic Usage

```bash
# Clone the repository
git clone https://github.com/LLyamk3i/m3u8-converter.git
cd m3u8-converter/experimentation

# Build the project
cargo build --release

# Run the converter on an M3U8 file
./target/release/experimentation /path/to/your/playlist.m3u8
```

### Advanced Usage

Enable verbose output to see details about each segment:

```bash
VERBOSE=1 ./target/release/experimentation /path/to/your/playlist.m3u8
```

### Output

The tool will create an MP4 file in the same directory as the input M3U8 file, with the same name but with the .mp4 extension.

## Project Structure

- **main.rs**: Entry point that orchestrates the overall conversion process
- **m3u8_parser.rs**: Contains the `M3u8Parser` that reads and parses M3U8 playlist files to extract segment URLs and codec information
- **m3u8_validator.rs**: Includes the `Segments` struct for validating the existence of referenced TS segments
- **ffmpeg_remuxer.rs**: Implements the `FfmpegRemuxer` which uses the FFmpeg libraries to convert TS segments to MP4

### Technical Details

The project uses:
- `regex` for parsing M3U8 files and extracting codec information
- `ffmpeg-next` (v6.0.0) for handling media conversion
- Robust error handling to deal with missing segments and conversion issues

## Development Status

This is an experimental project and may not handle all edge cases or M3U8 variants. Contributions and improvements are welcome!

## License

[Add your license information here]

