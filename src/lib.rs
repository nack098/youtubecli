pub mod youtubecli;
use youtubecli::process::start::*;

pub use crate::youtubecli::{
    option::parser::create_parameter_list,
    types::InputOptionList
};

pub fn run<'a>(args: &'a Vec<String>) -> Result<(), [String; 2]>{
    let inputs = youtubecli::option::parser::create_parameter_list(args.to_vec());
    match inputs {
        Ok(inputs) => {
            match start(&inputs){
                Ok(_) => (),
                Err(e) => return Err([e, "".to_string()])
            };
            Ok(())
        },
        Err(e) => Err(e)
    }
}