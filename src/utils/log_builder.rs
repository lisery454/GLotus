use chrono::Local;
use env_logger::{Builder, Target};
use log::LevelFilter;
use std::{
    env,
    fs::{self, File}, path::Path,
};

pub(crate) fn setup_logger() {
    let log_dir = "logs";

    let args: Vec<String> = env::args().collect();
    let exe_name = args
        .get(0)
        .and_then(|path_str| {
            let path = Path::new(path_str);
            path.file_stem()
        })
        .and_then(|os_str| os_str.to_str())
        .unwrap_or("unknown_example");
    fs::create_dir_all(log_dir).expect("Failed to create logs directory");

    let timestamp = Local::now().format("%Y-%m-%d_%H-%M-%S");
    let file_name = format!("{}/{}_{}.log", log_dir, timestamp, exe_name);

    let log_file =
        File::create(&file_name).expect(&format!("Failed to create log file: {}", file_name));

    Builder::new()
        .target(Target::Pipe(Box::new(log_file))) // 输出到文件
        .filter_level(LevelFilter::Info) // 设置默认日志级别
        .format_timestamp_secs() // 添加时间戳
        .init(); // 初始化全局 logger
}
