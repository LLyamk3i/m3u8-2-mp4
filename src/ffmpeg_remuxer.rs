use ffmpeg_next::format::{
    context::Output as OutputContext, input, output,
};
use ffmpeg_next::{media, Codec};
use std::path::PathBuf;

pub struct FfmpegRemuxer;

impl FfmpegRemuxer {
    pub fn new() -> Self {
        FfmpegRemuxer
    }

    pub fn remux(&self, segment_paths: &[PathBuf], output_path: &PathBuf) -> Result<(), String> {
        self.initialize_ffmpeg()?;
        let mut output_context = self.open_output_context(output_path)?;
        self.configure_output_streams(segment_paths, &mut output_context)?;
        self.write_output_header(&mut output_context)?;
        self.process_segments(segment_paths, &mut output_context)?;
        self.write_output_trailer(&mut output_context)?;

        Ok(())
    }

    fn initialize_ffmpeg(&self) -> Result<(), String> {
        ffmpeg_next::init().map_err(|e| format!("Failed to initialize ffmpeg: {}", e))
    }

    fn open_output_context(&self, output_path: &PathBuf) -> Result<OutputContext, String> {
        output(output_path).map_err(|e| {
            format!(
                "Failed to create output context for '{}': {}",
                output_path.display(),
                e
            )
        })
    }

    fn configure_output_streams(
        &self,
        segment_paths: &[PathBuf],
        output_ctx: &mut OutputContext,
    ) -> Result<(), String> {
        if let Some(first_segment_path) = segment_paths.first() {
            let ictx_first = input(first_segment_path).map_err(|e| {
                format!(
                    "Failed to open first segment '{}': {}",
                    first_segment_path.display(),
                    e
                )
            })?;

            for stream in ictx_first.streams() {
                let codec_params = stream.parameters();
                if codec_params.medium() == media::Type::Video
                    || codec_params.medium() == media::Type::Audio
                {
                    // It's important to use a codec here, not just the ID, if we were to set specific codec settings.
                    // However, for stream copying, parameters are usually sufficient.
                    // If ffmpeg-next has a more direct way to copy codec context/parameters, that would be better.
                    // For now, add_stream might take a Codec, but if we pass None or the existing codec_id, it should work for copying.
                    let mut out_stream = output_ctx
                        .add_stream(None::<Codec>)
                        .map_err(|e| format!("Failed to add stream to output context: {}", e))?;
                    // Convert input parameters (Ref) to owned Parameters to modify the tag
                    let mut output_params_owned = ffmpeg_next::codec::Parameters::from(codec_params);

                    // Explicitly set codec tag for MP4 compatibility
                    match output_params_owned.medium() {
                        media::Type::Video => {
                            if output_params_owned.id() == ffmpeg_next::codec::Id::H264 {
                                unsafe {
                                    (*output_params_owned.as_mut_ptr()).codec_tag = u32::from_le_bytes(*b"avc1");
                                }
                            }
                            // TODO: Add more video codec tags if necessary (e.g., HEVC -> "hvc1" or "hev1")
                        }
                        media::Type::Audio => {
                            if output_params_owned.id() == ffmpeg_next::codec::Id::AAC {
                                unsafe {
                                    (*output_params_owned.as_mut_ptr()).codec_tag = u32::from_le_bytes(*b"mp4a");
                                }
                            }
                            // TODO: Add more audio codec tags if necessary
                        }
                        _ => {}
                    }
                    // Set the modified parameters to the output stream
                    out_stream.set_parameters(output_params_owned);
                }
            }
            // ictx_first is dropped here, its parameters should have been copied to out_stream.
        } else {
            return Err("No input segments provided to configure streams".to_string());
        }
        Ok(())
    }

    fn write_output_header(&self, output_ctx: &mut OutputContext) -> Result<(), String> {
        output_ctx
            .write_header()
            .map_err(|e| format!("Failed to write output header: {}", e))
    }

    fn process_segments(
        &self,
        segment_paths: &[PathBuf],
        output_ctx: &mut OutputContext,
    ) -> Result<(), String> {
        // let mut last_dts_map: std::collections::HashMap<usize, i64> = std::collections::HashMap::new();
        // Proper DTS/PTS handling across concatenated segments can be very complex and depends on
        // whether segments are from a continuous stream or are independent.
        // FFmpeg CLI's concat demuxer or protocol handles this. Doing it manually here is involved.
        // This simplified loop assumes segments are somewhat aligned or that minor discontinuities are acceptable.

        for (segment_index, segment_path) in segment_paths.iter().enumerate() {
            let mut ictx = input(segment_path).map_err(|e| {
                format!(
                    "Segment {}: Failed to open '{}': {}",
                    segment_index,
                    segment_path.display(),
                    e
                )
            })?;

            for (stream, mut packet) in ictx.packets() {
                // Find the corresponding output stream by original codec ID and medium type
                let in_stream_params = stream.parameters();
                let out_stream_index = output_ctx.streams()
                    .position(|s| {
                        let out_s_params = s.parameters();
                        // This matching logic might need to be more robust if codec_tag or other specific parameters differ
                        // but should generally work if streams were set up from the first segment correctly.
                        out_s_params.id() == in_stream_params.id() && out_s_params.medium() == in_stream_params.medium()
                    })
                    .ok_or_else(|| format!("Segment {}: Could not find output stream for input stream {} (id: {:?}, medium: {:?})", 
                                           segment_index, stream.index(), in_stream_params.id(), in_stream_params.medium()))?;

                // TODO: Advanced timestamp recalculation if needed.
                // E.g., for video, something like:
                // if packet.dts().is_some() && stream.codec().medium() == media::Type::Video {
                //     let prev_dts = last_dts_map.entry(stream.index()).or_insert(0);
                //     packet.set_dts(Some(*prev_dts + calculated_duration_of_packet_or_fixed_offset));
                //     *prev_dts = packet.dts().unwrap();
                // }
                // Similar for PTS.
                // This is highly dependent on the source material and desired output behavior.
                // For now, we pass them through, which works for some TS concatenations.

                packet.set_stream(out_stream_index);
                packet.write_interleaved(output_ctx).map_err(|e| {
                    format!(
                        "Segment {}: Failed to write packet for stream {}: {}",
                        segment_index, out_stream_index, e
                    )
                })?;
            }
        }
        Ok(())
    }

    fn write_output_trailer(&self, output_ctx: &mut OutputContext) -> Result<(), String> {
        output_ctx
            .write_trailer()
            .map_err(|e| format!("Failed to write output trailer: {}", e))
    }
}
