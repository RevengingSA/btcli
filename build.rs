use std::env;

fn main() {
    // 获取当前构建的配置信息
    let target = env::var("TARGET").expect("TARGET environment variable not set");
    let profile = env::var("PROFILE").expect("PROFILE environment variable not set");
    
    // 仅在release模式且编译完成后执行UPX压缩
    if profile == "release" {
        // 使用环境变量控制是否执行UPX压缩
        let do_upx_compression = env::var("DO_UPX_COMPRESSION").unwrap_or_else(|_| "1".to_string());
        
        if do_upx_compression != "0" {
            // 构建完成后延迟执行UPX压缩
            let mut binary_path = std::path::PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
            binary_path.push("target/release/");
            
            // 确定可执行文件扩展名
            let exe_ext = if target.contains("windows") { ".exe" } else { "" };
            let binary_name = format!("{}{}", env::var("CARGO_PKG_NAME").unwrap(), exe_ext);
            binary_path.push(&binary_name);
            
            // 将UPX压缩作为一个后期构建步骤
            println!("cargo:warning=Post-build: To run UPX compression, execute 'cargo run --release --bin post_build'");
        }
    }
    
    println!("cargo:info=Build script executed for {} profile", profile);
}