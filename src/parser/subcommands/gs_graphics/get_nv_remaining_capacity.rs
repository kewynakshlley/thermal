use crate::parser::*;

#[derive(Clone)]
pub struct Handler;

impl CommandHandler for Handler {
  //TODO implement transmit
}

//Transmits the number of bytes of remaining memory (unused area) in the NV graphics area.
pub fn new() -> Command {
  Command::new(
    "Get NV Remaining Capacity",
    vec![4, 52], 
    CommandType::Subcommand,
    DataType::Subcommand,
    Box::new(Handler)
  )
}