use clap::ValueEnum;


#[derive(ValueEnum, Clone, Debug)]
pub enum Goal {
    Tokens,
    Hover,
    Definition,
    Execution
}
