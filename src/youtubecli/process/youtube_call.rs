// use std::{io::Read, process::{Command, Stdio}};
use subprocess::{
    Exec,
    ExitStatus,
    PopenError
};

pub fn call_yt_dlp(input_option: &Vec<String>) -> Result<ExitStatus, PopenError>{
    (yt_dlp(input_option) | ffplay()).join()
}

fn yt_dlp(input_option: &Vec<String>) -> Exec {
    Exec::cmd("yt-dlp")
        .args(input_option)
}

#[allow(dead_code)]
fn ffmpeg() -> Exec {
    Exec::cmd("ffmpeg")
        .arg("-i")
        .arg("pipe:")
        .arg("-f")
        .arg("matroska")
        .arg("-")
}

fn ffplay() -> Exec {
    Exec::cmd("ffplay")
        .arg("-i")
        .arg("pipe:")
        .arg("-autoexit")
}