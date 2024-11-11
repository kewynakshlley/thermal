use crate::{command::*, context::*, graphics::*};

#[derive(Clone)]
pub struct Handler;

impl CommandHandler for Handler {
    fn get_graphics(&self, command: &Command, context: &Context) -> Option<GraphicsCommand> {
        if let Some(img_ref) = ImageRef::from_data(&command.data, ImageRefStorage::Ram) {
            return context.graphics.stored_graphics.get(&img_ref).cloned();
        }
        None
    }
}

pub fn new() -> Command {
    Command::new(
        "Print Download (RAM) Graphics",
        vec![85],
        CommandType::Graphics,
        DataType::Subcommand,
        Box::new(Handler),
    )
}
