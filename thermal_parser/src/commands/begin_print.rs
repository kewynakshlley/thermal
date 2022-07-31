use crate::{command::*, constants::*};
use crate::context::Context;
use crate::graphics::GraphicsCommand;

#[derive(Clone)]
struct Handler;

impl CommandHandler for Handler {
    fn get_device_command(&self, _command: &Command, _context: &Context) -> Option<Vec<DeviceCommand>> {
        Some(vec![DeviceCommand::BeginPrint])
    }
}

pub fn new() -> Command {
    Command::new(
      "Begin Print",
      vec![],
      CommandType::Control,
      DataType::Empty,
      Box::new(Handler {}),
    )
}


//Arc::new(Handler{}
