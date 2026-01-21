use std::env;
use std::process::Command;

fn main() {
    println!("cargo: rerun-if-changed=res\\favicon.ico");
    #[cfg(windows)]
    {
        let mut res = winres::WindowsResource::new();
        // 设置图标路径（路径是相对于 build.rs 的）
        res.set_icon("res\\favicon.ico");
        res.compile().unwrap();
    }

    // 只在发布模式且是 Unix/Linux/macOS 下运行 (Windows 下 UPX 兼容性有时较差)
    if env::var("PROFILE").unwrap() == "release" {
        let target = env::var("CARGO_PKG_NAME").unwrap();
        let binary_path = format!("target/release/{}", target);

        // 尝试调用 UPX
        let status = Command::new("upx")
            .arg("--best") // 最佳压缩
            .arg(&binary_path)
            .status();

        match status {
            Ok(exit_status) if exit_status.success() => {
                println!("📦 UPX 压缩成功: {}", binary_path);
            }
            _ => {
                println!("⚠️  UPX 未安装或压缩失败。请安装 UPX 以获得更小的二进制文件。");
                println!("💡 安装命令 (macOS): brew install upx");
                println!("💡 安装命令 (Ubuntu): sudo apt-get install upx");
                println!("💡 安装命令 (Msys2): pacman install upx");
            }
        }
    }
}
