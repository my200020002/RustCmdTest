#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(unused_mut)]
#![allow(unused_attributes)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_must_use)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
use encoding_rs::GBK;
use std::convert::TryInto;
use std::env;
use std::io::{self, BufRead, BufReader, Read, Result};
use std::process::{Command, Stdio};

fn main() -> std::io::Result<()> {
    let mut process = Command::new(r"D:\python27\python27.exe")
        .arg(r"D:\volatility2_python\vol.py")
        .arg(r"--plugin=D:\volatility2_plugin")
        .arg(r"-f")
        .arg(r"C:/Users/<User>/Desktop/Work/abc.raw")
        .arg(r"--profile=Win7SP1x64")
        .arg("procdump")
        .arg("-p")
        .arg("17008")
        .arg("--dump-dir=output")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    // 使用 as_mut() 来借用 stderr，而不是移动它
    let stdout = process.stdout.take().expect("Failed to get stdout");
    let stderr = process.stderr.take().expect("Failed to get stderr");

    let stdout_reader = BufReader::new(stdout);
    let stderr_reader = BufReader::new(stderr);

    // 读取标准输出
    std::thread::spawn(move || {
        for line in stdout_reader.lines() {
            if let Ok(line) = line {
                println!("STDOUT: {}", line);
            }
        }
    });
    std::thread::spawn(move || {
        for line in stderr_reader.lines() {
            if let Ok(line) = line {
                println!("STDERR:{}", line);
            }
        }
    });

    // 等待进程结束
    let exit_status = process.wait()?;
    println!("Process exited with: {}", exit_status);

    Ok(())
}
