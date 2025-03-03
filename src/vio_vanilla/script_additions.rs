use crate::vio::ViolaDefault;
use std::fs;
use std::sync::{Arc, RwLock};
use askama::Template;
use viola::ViolaDefault;
use crate::pack::{Pack, PackPart};
use crate::script::ScriptAddition;
use crate::vio::Buildable;

const COMMAND_PATH: &str = "viogen_commands";

#[derive(Clone)]
pub enum CustomCommandArg {
    Num,
    Str
}

#[derive(Clone, ViolaDefault)]
#[derive(Default)]
pub struct CustomCommand {
    pub prefix: String,
    pub name: String,
    pub path: String,
    pub args: Vec<CustomCommandArg>
}

impl Buildable for CustomCommand {}

#[derive(Template)]
#[template(
    path = "scripts/command_setup.js.jinja2",
    escape = "none"
)]
struct CommandTemplate {
    args: String,
    argtypes: String,
    path: String,
    prefix: String
}

impl ScriptAddition for CustomCommand {
    fn build_addition(&self, pack: Arc<RwLock<&&Pack>>) {
        let mut main = pack.read().unwrap()
            .read_file(PackPart::BP, "scripts/main.js")
            .unwrap();
        
        main.push_str(
            &format!("\nimport \"./{}/{}\"", COMMAND_PATH, &self.name)
        );
        
        pack.read().unwrap()
            .write_file(PackPart::BP, "scripts/main.js", main);
        

        let template = CommandTemplate {
            args: {
                let mut args = String::new();

                for (index, _) in self.args.iter().enumerate() {
                    args.push_str(&format!("args[{}],", index));
                }

                args.pop();

                args
            },
            path: format!("../viogen_commands_user/{}", &self.name),
            prefix: format!("{}{}", self.prefix, self.name),
            argtypes: {
                let mut res = String::new();

                for arg in self.args.iter() {
                    let arg_type = if let CustomCommandArg::Str = arg {
                        "string"
                    } else {
                        "number"
                    };
                    res.push_str(&format!("\"{}\",", arg_type));
                }

                res.pop();

                format!("[{}]", res)
            }
        }.render().unwrap();

        pack.read().unwrap()
            .write_file(PackPart::BP, format!("scripts/viogen_commands/{}.js", self.name), template);

        pack.read().unwrap()
            .write_file(PackPart::BP, format!("scripts/viogen_commands_user/{}.js", self.name),
                fs::read_to_string(&self.path).unwrap()
            );
    }
}

impl CustomCommand {
    pub fn new(
        prefix: impl Into<String>,
        name: impl Into<String>,
        path: impl Into<String>,
        args: Vec<CustomCommandArg>
    ) -> Self {
        Self {
            prefix: prefix.into(),
            name: name.into(),
            path: path.into(),
            args
        }
    }
}