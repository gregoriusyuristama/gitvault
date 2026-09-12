//! Archive module: tar + zstd streaming compression/decompression.

use anyhow::{Context, Result};
use std::io::{Read, Write};
use std::path::Path;

/// Pack `source_dir` into `output` as a tar stream wrapped in zstd compression.
pub fn pack_directory<W: Write>(source_dir: &Path, output: W) -> Result<()> {
    let zstd_encoder = zstd::stream::Encoder::new(output, 3)
        .context("failed to initialize zstd encoder")?
        .auto_finish();
    let mut tar_builder = tar::Builder::new(zstd_encoder);
    tar_builder
        .append_dir_all(".", source_dir)
        .with_context(|| format!("failed to package directory {}", source_dir.display()))?;
    tar_builder.finish().context("failed to finalize tar")?;
    Ok(())
}

/// Unpack a zstd-compressed tar stream from `input` into `target_dir`.
pub fn unpack_directory<R: Read>(input: R, target_dir: &Path) -> Result<()> {
    std::fs::create_dir_all(target_dir)
        .with_context(|| format!("failed to create target dir {}", target_dir.display()))?;
    let zstd_decoder =
        zstd::stream::Decoder::new(input).context("failed to initialize zstd decoder")?;
    let mut archive = tar::Archive::new(zstd_decoder);
    archive
        .unpack(target_dir)
        .with_context(|| format!("failed to unpack into {}", target_dir.display()))?;
    Ok(())
}
