use std::path::Path;

/// Ensure directory exists, create if not
pub async fn ensure_dir(path: &Path) -> anyhow::Result<()> {
    if !path.exists() {
        tokio::fs::create_dir_all(path).await?;
    }
    Ok(())
}

/// Copy directory recursively
pub async fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> anyhow::Result<()> {
    let src = src.as_ref();
    let dst = dst.as_ref();
    
    tokio::fs::create_dir_all(&dst).await?;
    
    let mut entries = tokio::fs::read_dir(src).await?;
    
    while let Some(entry) = entries.next_entry().await? {
        let ty = entry.file_type().await?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());
        
        if ty.is_dir() {
            Box::pin(copy_dir_all(&src_path, &dst_path)).await?;
        } else {
            tokio::fs::copy(&src_path, &dst_path).await?;
        }
    }
    
    Ok(())
}
