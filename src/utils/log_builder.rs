use std::fs::File;

use env_logger::{Builder, Target};
use log::LevelFilter;

pub(crate) fn setup_logger() {
    // 创建或打开当前目录的日志文件
    let log_file = File::create("app.log").expect("Failed to create log file");

    // 配置 env_logger
    Builder::new()
        .target(Target::Pipe(Box::new(log_file))) // 输出到文件
        .filter_level(LevelFilter::Info) // 设置默认日志级别
        .format_timestamp_secs() // 添加时间戳
        .init(); // 初始化全局 logger
}
