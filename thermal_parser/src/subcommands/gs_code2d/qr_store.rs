extern crate qr_code;

use crate::context::QrModel::Micro;
use crate::{command::*, context::*, graphics};
use qr_code::{EcLevel, QrCode, Version};

#[derive(Clone)]
pub struct Handler;

impl CommandHandler for Handler {
    fn apply_context(&self, command: &Command, context: &mut Context) {
        let data = command.data.to_owned();

        let version = match &context.code2d.qr_model {
            QrModel::Model1 => Version::Normal(1),
            QrModel::Model2 => Version::Normal(2),
            Micro => Version::Micro(4),
        };

        let error_correction = match context.code2d.qr_error_correction {
            QrErrorCorrection::M => EcLevel::M,
            QrErrorCorrection::Q => EcLevel::Q,
            QrErrorCorrection::H => EcLevel::H,
            _ => EcLevel::L,
        };

        let result = QrCode::with_version(data, version, error_correction);

        match result {
            Ok(qr) => {
                let raw = qr.to_vec();
                let mut converted_points = Vec::<u8>::with_capacity(raw.capacity());

                for b in raw {
                    let v = if b { 1 } else { 0 };
                    converted_points.push(v);
                }

                let qrcode = graphics::Code2D {
                    points: converted_points,
                    width: qr.width() as u32,
                    point_width: context.code2d.qr_size as u32,
                    point_height: context.code2d.qr_size as u32,
                };

                context.code2d.symbol_storage = Some(qrcode);
            }
            Err(e) => {
                println!("QR ERROR {}", e);
            }
        }
    }
}

pub fn new() -> Command {
    Command::new(
        "QR Store the Code2D data",
        vec![49, 80],
        CommandType::Context,
        DataType::Subcommand,
        Box::new(Handler),
    )
}
