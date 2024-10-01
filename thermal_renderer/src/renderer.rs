use thermal_parser::command::{Command, CommandType, DeviceCommand};
use thermal_parser::context::{Context, HumanReadableInterface};
use thermal_parser::graphics::GraphicsCommand;

pub trait CommandRenderer {
    //default implementation
    fn process_command(&mut self, context: &mut Context, command: &Command) {
        match command.kind {
            CommandType::Text => {
                let maybe_text = command.handler.get_text(command, context);
                if let Some(text) = maybe_text {
                    self.draw_text(context, text);
                }
            }
            CommandType::Graphics => {
                let maybe_gfx = command.handler.get_graphics(command, context);

                if let Some(gfx) = maybe_gfx {
                    if context.page_mode.enabled {
                        self.handle_graphics(&gfx, context);
                    } else {
                        self.handle_page_mode_graphics(&gfx, context);
                    }
                }
            }
            CommandType::Context => {
                command.handler.apply_context(command, context);
            }
            CommandType::ContextControl => {
                command.handler.apply_context(command, context);

                self.handle_device_commands(
                    &command.handler.get_device_command(command, context),
                    context,
                );
            }
            CommandType::Control => {
                self.handle_device_commands(
                    &command.handler.get_device_command(command, context),
                    context,
                );
            }
            _ => {}
        }
    }

    fn handle_device_commands(
        &mut self,
        device_commands: &Option<Vec<DeviceCommand>>,
        context: &mut Context,
    ) {
        if let Some(device_commands) = device_commands {
            for device_command in device_commands {
                self.draw_device_command(context, device_command);

                match device_command {
                    DeviceCommand::BeginPrint => self.begin_render(context),
                    DeviceCommand::EndPrint => self.end_render(context),
                    DeviceCommand::FeedLine(num_lines) => {
                        let advance = context.line_height_pixels() as usize * *num_lines as usize;

                        if context.page_mode.enabled {
                            context.page_mode.y += advance;
                        } else {
                            context.graphics.y += advance;

                        }
                    }
                    DeviceCommand::Feed(num) => {
                        let advance = context.motion_unit_y_pixels() as usize * *num as usize;

                        if context.page_mode.enabled {
                            context.page_mode.y += advance;
                        } else {
                            context.graphics.y += advance;
                        }
                    }
                    DeviceCommand::FullCut | DeviceCommand::PartialCut => {
                        context.graphics.y += context.line_height_pixels() as usize * 2;
                    }
                    DeviceCommand::BeginPageMode => {
                        context.page_mode.enabled = true;
                        self.begin_page(context);
                    }
                    DeviceCommand::EndPageMode(print) => {
                        self.end_page(context, *print);
                        context.page_mode.enabled = false
                    }
                    DeviceCommand::ChangePageModeDirection => {
                        self.page_direction_changed(context);
                    }
                    DeviceCommand::ChangePageArea => {
                        self.page_area_changed(context);
                    }
                    _ => {}
                }
            }
        }
    }

    fn handle_graphics(&mut self, gfx: &GraphicsCommand, context: &mut Context) {
        match gfx {
            GraphicsCommand::Code2D(code_2d) => {
                self.begin_graphics(context);

                let mut i = 1;
                let origin_x = context.graphics_x_offset(
                    (code_2d.points.len() * code_2d.point_width as usize) as u32,
                ) as usize;

                for p in &code_2d.points {
                    if i != 1 && i % code_2d.width == 1 {
                        context.graphics.x = origin_x;
                        context.graphics.y += code_2d.point_height as usize;
                    }

                    if *p > 0 {
                        self.draw_rect(
                            context,
                            code_2d.point_width as usize,
                            code_2d.point_height as usize,
                        )
                    }
                    context.graphics.x += code_2d.point_width as usize;
                    i += 1;
                }

                context.graphics.x = 0;

                self.end_graphics(context);
            }
            GraphicsCommand::Barcode(barcode) => {
                match context.barcode.human_readable {
                    HumanReadableInterface::Above | HumanReadableInterface::Both => {
                        self.draw_text(context, barcode.text.to_string());
                    }
                    _ => {}
                }

                self.begin_graphics(context);

                context.graphics.x = context.graphics_x_offset(
                    (barcode.points.len() * barcode.point_width as usize) as u32,
                ) as usize;

                for p in &barcode.points {
                    if *p > 0 {
                        self.draw_rect(
                            context,
                            barcode.point_width as usize,
                            barcode.point_height as usize,
                        )
                    }
                    context.graphics.x += barcode.point_width as usize;
                }

                context.graphics.x = 0;
                context.graphics.y += barcode.point_height as usize;
                context.graphics.y += context.line_height_pixels() as usize;

                self.end_graphics(context);

                match context.barcode.human_readable {
                    HumanReadableInterface::Below | HumanReadableInterface::Both => {
                        self.draw_text(context, barcode.text.to_string());
                    }
                    _ => {}
                }
            }
            GraphicsCommand::Image(image) => {
                if image.advances_xy {
                    context.graphics.x =
                        context.graphics_x_offset(image.width) as usize;
                }
                self.draw_image(
                    context,
                    image.as_grayscale(),
                    image.width as usize,
                    image.height as usize,
                );
                if image.advances_xy {
                    context.graphics.x = 0;
                    context.graphics.y += image.height as usize;
                    context.graphics.y += context.line_height_pixels() as usize;
                }
            }
            GraphicsCommand::Rectangle(_) => {}
            GraphicsCommand::Line(_) => {}
        }
    }

    fn handle_page_mode_graphics(&mut self, gfx: &GraphicsCommand, context: &mut Context){
        match gfx {
            GraphicsCommand::Code2D(code_2d) => {
                self.begin_graphics(context);

                let mut i = 1;
                let origin_x = context.page_mode.x;

                for p in &code_2d.points {
                    if i != 1 && i % code_2d.width == 1 {
                        context.page_mode.x = origin_x;
                        context.page_mode.y += code_2d.point_height as usize;
                    }

                    if *p > 0 {
                        self.draw_rect(
                            context,
                            code_2d.point_width as usize,
                            code_2d.point_height as usize,
                        )
                    }
                    context.page_mode.x += code_2d.point_width as usize;
                    i += 1;
                }

                context.page_mode.x = 0;

                self.end_graphics(context);
            }
            GraphicsCommand::Barcode(barcode) => {
                match context.barcode.human_readable {
                    HumanReadableInterface::Above | HumanReadableInterface::Both => {
                        self.draw_text(context, barcode.text.to_string());
                    }
                    _ => {}
                }

                self.begin_graphics(context);

                context.graphics.x = context.graphics_x_offset(
                    (barcode.points.len() * barcode.point_width as usize) as u32,
                ) as usize;

                for p in &barcode.points {
                    if *p > 0 {
                        self.draw_rect(
                            context,
                            barcode.point_width as usize,
                            barcode.point_height as usize,
                        )
                    }
                    context.graphics.x += barcode.point_width as usize;
                }

                context.graphics.x = 0;
                context.graphics.y += barcode.point_height as usize;
                context.graphics.y += context.line_height_pixels() as usize;

                self.end_graphics(context);

                match context.barcode.human_readable {
                    HumanReadableInterface::Below | HumanReadableInterface::Both => {
                        self.draw_text(context, barcode.text.to_string());
                    }
                    _ => {}
                }
            }
            GraphicsCommand::Image(image) => {
                if image.advances_xy {
                    context.page_mode.x = 0;
                }
                self.draw_image(
                    context,
                    image.as_grayscale(),
                    image.width as usize,
                    image.height as usize,
                );
                if image.advances_xy {
                    context.page_mode.x = 0;
                    context.page_mode.y += image.height as usize;
                    context.page_mode.y += context.line_height_pixels() as usize;
                }
            }
            GraphicsCommand::Rectangle(_) => {}
            GraphicsCommand::Line(_) => {}
        }
    }

    fn begin_render(&mut self, context: &mut Context);
    fn begin_page(&mut self, context: &mut Context);
    fn page_area_changed(&mut self, context: &mut Context);
    fn page_direction_changed(&mut self, context: &mut Context);
    fn end_page(&mut self, context: &mut Context, print: bool);
    fn begin_graphics(&mut self, context: &mut Context);
    fn draw_rect(&mut self, context: &mut Context, w: usize, h: usize);
    fn end_graphics(&mut self, context: &mut Context);
    fn draw_image(&mut self, context: &mut Context, bytes: Vec<u8>, width: usize, height: usize);
    fn draw_text(&mut self, context: &mut Context, text: String);
    fn draw_device_command(&mut self, context: &mut Context, command: &DeviceCommand);
    fn end_render(&mut self, context: &mut Context);
}
