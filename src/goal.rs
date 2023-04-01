use clap::ValueEnum;


#[derive(ValueEnum, Clone, Debug)]
pub enum Goal {
    Tokens,
    Hover,
    Definition,
    Execution
}

impl Goal {
    pub fn to_string(&self) -> &str {
        match self {
            Goal::Tokens => "tokens",
            Goal::Hover => "hover",
            Goal::Definition => "definition",
            Goal::Execution => "execution"
        }
    }
}
