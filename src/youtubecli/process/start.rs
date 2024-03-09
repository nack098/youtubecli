use std::process::exit;

use colored::Colorize;

use crate::youtubecli::{
    process::youtube_call, types::{
        InputOptionList, ParameterCommandType, ParameterType, SingleCommandType
    }
};

pub fn start(inputs: &InputOptionList) -> Result<(), String>{
    let mut param = vec!["-o".to_string(), "-".to_string()];
    for input in inputs.0.iter() {
        match input.parameter_type {
            ParameterType::ParameterCommand(ParameterCommandType::FORMAT) => {
                // param.push("-f".to_string());
                // param.push(input.value.clone());
            },
            ParameterType::ParameterCommand(ParameterCommandType::QUALITY) => {
                param.push("-f".to_string());
                param.push(input.value.clone());
            },
            ParameterType::ParameterCommand(ParameterCommandType::URL) => {
                param.push(input.value.clone());
            },
            ParameterType::ParameterCommand(ParameterCommandType::Search) => {
                param.push(format!("ytsearch:{}", input.value.clone()))
            },
            ParameterType::SingleCommand(SingleCommandType::HELP) => {
                println!("{}", 
                    "Usage: youtubecli [OPTIONS] \n
                     Option list: \n
                     -q, --quality [QUALITY] \t Set the quality of the video \n
                     -s, --search [SEARCH] \t Search for a video \n
                     -u, --url [URL] \t Download a video from a URL \n
                     -h, --help \t Show this message \n
                     -v, --version \t Show the version of the program \n
                     -x, --silent \t Do nothing \n
                     \n
                     Example: 
                     Search:\tyoutubecli -q best -s 'Polka - Everblue'\n
                     URL:\tyoutubecli -q best -u 'https://www.youtube.com/watch?v=3JZ_D3ELwOQ'\n".blue()
                );
                exit(0)
            },
            ParameterType::SingleCommand(SingleCommandType::VERSION) => {
                println!("Youtube From Command Line {}", env!("CARGO_PKG_VERSION").blue());
                exit(0)
            },
            ParameterType::SingleCommand(SingleCommandType::SILENT) => {
                println!("Doing nothing...");
                exit(0)
            }
            _ => ()
        }
    }
    match youtube_call::call_yt_dlp(&param) {
        Ok(_) => {},
        Err(e) => return Err(format!("Error: {}", e))
    };

    Ok(())
}