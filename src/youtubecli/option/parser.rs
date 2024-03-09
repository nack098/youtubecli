use crate::youtubecli::{
    types::{
        InputOption, 
        InputOptionList, 
        ParameterType, 
        SingleCommandType, 
        ParameterCommandType
    },
    option::settings::Settings
};


fn param_error_message<'a>(parameter_type: &'a ParameterType) -> [String; 2] {
    let settings = Settings::new();
    match parameter_type {
        ParameterType::ParameterCommand(ParameterCommandType::FORMAT) => {
            ["ERROR!!: Invalid Format Value".to_string(),
                format!("Allowed formats: {:?}\n", settings.parameter_commands.allow_format).to_string()]
        },
        ParameterType::ParameterCommand(ParameterCommandType::QUALITY) => {
            ["ERROR!!: Invalid Quality Value".to_string(),
                format!("Allowed qualities: {:?}\n", settings.parameter_commands.allow_quality)]
        },
        ParameterType::ParameterCommand(ParameterCommandType::URL) => {
            ["ERROR!!: Invalid URL Value".to_string(),
                "Please provide a valid URL\n
                Example: https://www.youtube.com/watch?v=6g4dkBF5anU\n
                If you want to search, please use -s or --search\n".to_string()]
        },
        ParameterType::ParameterCommand(ParameterCommandType::Search) => {
            ["ERROR!!: Invalid Search Value".to_string(),
                "Please provide a valid search query\n
                Example: -s 'search query' or --search 'search query'\n
                If you want to use URL, please use -u or --url\n".to_string()]
        },
        _ => {
            ["ERROR!!: Invalid Parameter".to_string(),
                "Please check avalable commands\n
                via -h or --help\n".to_string()]
        }
    }
} 

pub fn create_parameter_list(raw_args: Vec<String>) -> Result<InputOptionList, [String; 2]> {
    let mut parameter_list = Vec::new();
    let mut current_param:ParameterType = ParameterType::Null;
    let args :Vec<String> = raw_args[1..].to_vec();
    if args.len() == 0 {
        return Err(["ERROR!!: No Arguments Provided".to_string(), "Please check the command with -h or --help\n".to_string()]);
    }
    for value in args.iter() {
        match Settings::get_parameter_type(value) {
            Ok(parameter_type) => {
                if parameter_type != ParameterType::SingleCommand(SingleCommandType::HELP)
                    && parameter_type != ParameterType::SingleCommand(SingleCommandType::VERSION)
                    && parameter_type != ParameterType::SingleCommand(SingleCommandType::SILENT)
                {
                    current_param=parameter_type;
                } else {
                    parameter_list.push(InputOption::new(parameter_type, "".to_string()));
                }
            },
            Err(_) => {
                if Settings::value_validator(value, current_param) {
                    parameter_list.push(InputOption::new(current_param, value.to_string()));
                } else {
                    return Err(param_error_message(&current_param));
                }
            }
        }
    }
    Ok(InputOptionList(parameter_list))
}