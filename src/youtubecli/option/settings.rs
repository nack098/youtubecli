use crate::youtubecli::types::{ParameterCommandType, ParameterType, SingleCommandType};

pub struct SingleCommands {
    pub help: Vec<String>,
    pub version: Vec<String>,
    pub silent: Vec<String>
}

pub struct ParameterCommands {
    pub format_param: Vec<String>,
    pub quality_param: Vec<String>,
    pub url_param: Vec<String>,
    pub search_param: Vec<String>,
    pub allow_format: Vec<String>,
    pub allow_quality: Vec<String>,
    
}

pub struct Settings {
    pub single_commands: SingleCommands,
    pub parameter_commands: ParameterCommands
}

impl Settings {
    pub fn new() -> Settings {
        Settings {
            parameter_commands: ParameterCommands{
                format_param: vec!["-f".to_string(), "--format".to_string()],
                quality_param: vec!["-q".to_string(), "--quality".to_string()],
                url_param: vec!["-u".to_string(), "--url".to_string()],
                search_param: vec!["-s".to_string(), "--search".to_string()],
                allow_format: vec!["mp3".to_string(), "mp4".to_string()],
                allow_quality: vec!["best".to_string(), "bestaudio".to_string(), "bestvideo".to_string()],
            },
            single_commands: SingleCommands{
                help: vec!["-h".to_string(), "--help".to_string()],
                version: vec!["-v".to_string(), "--version".to_string()],
                silent: vec!["-x".to_string(), "--silent".to_string()]
            }
        }
    }

    pub fn get_parameter_type(parameter: &String) -> Result<ParameterType, String> {
        let settings = Settings::new();
        match parameter {
            v if settings.parameter_commands.format_param.contains(&v) => Ok(ParameterType::ParameterCommand(ParameterCommandType::FORMAT)),
            v if settings.parameter_commands.quality_param.contains(&v) => Ok(ParameterType::ParameterCommand(ParameterCommandType::QUALITY)),
            v if settings.parameter_commands.url_param.contains(&v) => Ok(ParameterType::ParameterCommand(ParameterCommandType::URL)),
            v if settings.parameter_commands.search_param.contains(&v) => Ok(ParameterType::ParameterCommand(ParameterCommandType::Search)),
            v if settings.single_commands.help.contains(&v) => Ok(ParameterType::SingleCommand(SingleCommandType::HELP)),
            v if settings.single_commands.silent.contains(&v) => Ok(ParameterType::SingleCommand(SingleCommandType::SILENT)),
            v if settings.single_commands.version.contains(&v) => Ok(ParameterType::SingleCommand(SingleCommandType::VERSION)),
            _ => Err("Invalid parameter".to_string())
        }
    }

    pub fn value_validator(value: &String, parameter_type: ParameterType) -> bool {
        let settings = Settings::new();
        match parameter_type {
            ParameterType::ParameterCommand(ParameterCommandType::FORMAT) => settings.parameter_commands.allow_format.contains(value),
            ParameterType::ParameterCommand(ParameterCommandType::QUALITY) => settings.parameter_commands.allow_quality.contains(value),
            ParameterType::ParameterCommand(ParameterCommandType::URL) => {
                let url = url::Url::parse(value);
                match url {
                    Ok(_) => true,
                    Err(_) => false
                }
            },
            ParameterType::ParameterCommand(ParameterCommandType::Search) => {
                let url = url::Url::parse(value);
                match url {
                    Ok(_) => false,
                    Err(_) => true
                
                }
            },
            ParameterType::SingleCommand(_) => true,
            ParameterType::Null => false
        }
    }
}