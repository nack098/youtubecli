#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ParameterType {
    SingleCommand(SingleCommandType),
    ParameterCommand(ParameterCommandType),
    Null
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ParameterCommandType {
    FORMAT,
    URL,
    QUALITY,
    Search,
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SingleCommandType {
    HELP,
    VERSION,
    SILENT
}

#[derive(Debug)]
pub struct InputOption {
    pub parameter_type: ParameterType,
    pub value: String
}

impl InputOption {
    pub fn new(parameter_type: ParameterType, value: String) -> InputOption {
        InputOption {
            parameter_type,
            value
        }
    }
}

#[derive(Debug)]
pub struct InputOptionList(pub Vec<InputOption>);