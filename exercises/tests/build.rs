use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    // 获取当前时间戳
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();

    // 设置环境变量 TEST_FOO 为当前时间戳
    println!("cargo:rustc-env=TEST_FOO={}", timestamp);

    println!("cargo:rustc-cfg=feature=\"pass\"");

}
