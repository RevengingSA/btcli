use std::env;
use std::process::Command;
use std::path::Path;
use std::fs;

fn main() {
    // 获取项目根目录和二进制文件路径
    let mut binary_path = env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| ".".to_string());
    binary_path.push_str("/target/release/");
    
    // 根据操作系统确定二进制文件扩展名
    let exe_ext = if cfg!(windows) { ".exe" } else { "" };
    let binary_name = format!("{}{}", env::var("CARGO_PKG_NAME").unwrap_or("btcli".to_string()), exe_ext);
    binary_path.push_str(&binary_name);

    // 检查二进制文件是否存在
    if !Path::new(&binary_path).exists() {
        eprintln!("Binary file does not exist: {}", binary_path);
        return;
    }

    // 获取原始文件大小
    let original_size = fs::metadata(&binary_path).map(|m| m.len()).unwrap_or(0);
    println!("Original binary size: {} bytes", original_size);

    // 检查UPX是否可用
    let upx_check = Command::new("upx").arg("--version").output();
    if upx_check.is_ok() && upx_check.unwrap().status.success() {
        println!("Attempting to compress binary with UPX: {}", binary_path);
        
        // 使用UPX压缩，强制覆盖已有压缩
        let upx_output = Command::new("upx")
            .args(&["--best", "--lzma", "--force", &binary_path])
            .output();

        match upx_output {
            Ok(output) => {
                let stderr = String::from_utf8_lossy(&output.stderr);
                
                // 检查是否成功或已经压缩
                if output.status.success() || stderr.contains("AlreadyPackedException") {
                    // 获取压缩后的文件大小
                    let compressed_size = fs::metadata(&binary_path).map(|m| m.len()).unwrap_or(0);
                    let reduction = ((original_size as f64 - compressed_size as f64) / original_size as f64 * 100.0).round();
                    
                    if output.status.success() {
                        println!("✓ Successfully compressed binary with UPX");
                        println!("  Size reduced from {} to {} bytes ({:.1}% reduction)", 
                                original_size, compressed_size, reduction);
                    } else if stderr.contains("AlreadyPackedException") {
                        println!("⚠ Binary was already UPX compressed");
                        println!("  Current size: {} bytes", compressed_size);
                    }
                    
                    // 显示UPX详细信息
                    let _ = Command::new("upx").args(&["-q", &binary_path]).status();
                } else {
                    eprintln!("Failed to compress binary with UPX: {}", stderr);
                }
            }
            Err(e) => {
                eprintln!("Error executing UPX: {}", e);
            }
        }
    } else {
        println!("UPX not found, skipping compression");
    }
}