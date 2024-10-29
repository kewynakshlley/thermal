//! Image Renderer
//!
//! The image renderer renders all output to a single raster image.
//!
//! A lot of the heavy lifting is done by the thermal_image module.
//!
//! A note on Page Mode:
//!
//! Page mode is a more complex way of creating print data.
//!
//! It allows different print directions to be set. In order
//! to make this work, we actually rotate our image and coordinates
//! whenever the page mode direction is changed and then render as usual
//!
//! Page mode shares a lot of context from the main context, but it
//! has some of its own as well.
//!

use crate::image_renderer::thermal_image::ThermalImage;
use crate::renderer::{OutputRenderer, RenderOutput, Renderer};
use thermal_parser::context::{Context, PrintDirection, Rotation, TextJustify};
use thermal_parser::graphics::{Image, VectorGraphic};
use thermal_parser::text::TextSpan;

pub mod thermal_image;

pub struct ImageRenderer {
    pub paper_image: ThermalImage,
    pub page_image: ThermalImage,
    pub debug: bool,
}

impl ImageRenderer {
    pub fn new() -> Self {
        Self {
            paper_image: ThermalImage::new(0),
            page_image: ThermalImage::new(0),
            debug: false,
        }
    }

    /// This is the normal way to render bytes to an image
    pub fn render(bytes: &Vec<u8>) -> RenderOutput<ReceiptImage> {
        let mut image_renderer: Box<dyn OutputRenderer<_>> = Box::new(ImageRenderer::new());
        let mut renderer = Renderer::new(&mut image_renderer);
        renderer.render(bytes)
    }

    pub fn render_debug(bytes: &Vec<u8>) -> RenderOutput<ReceiptImage> {
        let mut img = ImageRenderer::new();
        img.debug = true;
        let mut image_renderer: Box<dyn OutputRenderer<_>> = Box::new(img);
        let mut renderer = Renderer::new(&mut image_renderer);
        renderer.debug = true;
        renderer.render(bytes)
    }
}

/// ReceiptImage is the main output for the image renderer
pub struct ReceiptImage {
    pub bytes: Vec<u8>,
    pub width: u32,
    pub height: u32,
}

impl OutputRenderer<ReceiptImage> for ImageRenderer {
    fn begin_render(&mut self, context: &mut Context) {
        if self.debug { println!("Begin render"); }
        //self.paper_image.enable_debug();

        //Initialize the main image area
        self.paper_image.empty();
        self.paper_image.set_width(context.graphics.render_area.w);
        self.paper_image.set_character_size(
            context.text.character_width as u32,
            context.text.character_height as u32,
        );

        //Initialize image area for page mode
        self.page_image.set_width(0);
        self.page_image.set_character_size(
            context.text.character_width as u32,
            context.text.character_height as u32,
        );

        //Page images should not auto grow in either direction
        //Normally only the width is locked down, but for page mode
        //We want to lock down the height as well
        self.page_image.auto_grow = false;
    }

    fn page_begin(&mut self, _context: &mut Context) {
        if self.debug { println!("Begin page"); }
        self.page_image.set_width(0);
    }

    fn page_area_changed(
        &mut self,
        _context: &mut Context,
        rotation: Rotation,
        width: u32,
        height: u32,
    ) {
        if self.debug { println!("Page Area Changed {:?} w{} h{}", rotation, width, height ); }

        let img = &mut self.page_image;

        match rotation {
            Rotation::R90 => img.rotate_90(),
            Rotation::R180 => img.rotate_180(),
            Rotation::R270 => img.rotate_270(),
            _ => {}
        }

        if width > self.page_image.width {
            self.page_image.expand_to_width(width)
        }
        if height > self.page_image.get_height() {
            self.page_image.expand_to_height(height)
        }
    }

    fn render_page(&mut self, context: &mut Context) {
        if self.debug { println!("Render Page"); }
        let rotation_to_standard = context.page_mode.calculate_directional_rotation(
            &context.page_mode.direction,
            &PrintDirection::TopLeft2Right,
        );

        //Rotate to standard direction
        match rotation_to_standard {
            Rotation::R90 => self.page_image.rotate_90(),
            Rotation::R180 => self.page_image.rotate_180(),
            Rotation::R270 => self.page_image.rotate_270(),
            _ => {}
        }

        let (w, h, pixels) = self.page_image.copy();

        //Rotate back to how it was
        let rotation_to_previous = context.page_mode.calculate_directional_rotation(
            &PrintDirection::TopLeft2Right,
            &context.page_mode.direction,
        );

        match rotation_to_previous {
            Rotation::R90 => self.page_image.rotate_90(),
            Rotation::R180 => self.page_image.rotate_180(),
            Rotation::R270 => self.page_image.rotate_270(),
            _ => {}
        }

        self.paper_image.put_pixels(
            context.graphics.render_area.x,
            context.graphics.render_area.y,
            w,
            h,
            pixels,
            false,
            false,
        );
    }

    fn render_graphics(&mut self, context: &mut Context, graphics: &Vec<VectorGraphic>) {
        if self.debug { println!("Render Graphics {:?}", graphics); }

        let page = context.page_mode.enabled;

        for graphic in graphics {
            match graphic {
                VectorGraphic::Rectangle(rectangle) => {
                    if page {
                        self.page_image.put_rect(rectangle);
                    } else {
                        self.paper_image.put_rect(rectangle);
                    }
                }
            }
        }
    }

    fn render_image(&mut self, context: &mut Context, image: &Image) {
        if self.debug { println!("Render Image x{} y{} w{} h{}", image.x, image.y, image.w, image.h); }
        if context.page_mode.enabled {
            self.page_image.put_render_img(image);
        } else {
            self.paper_image.put_render_img(image);
        }
    }

    fn render_text(
        &mut self,
        context: &mut Context,
        spans: &Vec<TextSpan>,
        x_offset: u32,
        max_height: u32,
        _text_justify: TextJustify,
    ) {
        if self.debug { println!("Render text {:?}", spans) }
        let canvas = if context.page_mode.enabled {
            &mut self.page_image
        } else {
            &mut self.paper_image
        };

        for span in spans {
            if let Some(_) = &span.dimensions {
                canvas.render_span(x_offset, max_height, span);
            }
        }
    }

    fn get_render_errors(&mut self) -> Vec<String> {
        let mut errors = vec![];
        let paper_errors = &self.paper_image.errors;
        let page_errors = &self.page_image.errors;

        for paper_error in paper_errors {
            errors.push(paper_error.to_owned());
        }

        for page_error in page_errors {
            errors.push(page_error.to_owned());
        }

        errors
    }

    fn end_render(&mut self, context: &mut Context) -> ReceiptImage {
        if self.debug { println!("End Render") }
        //Add in the left and right margin;
        self.paper_image
            .expand_to_width(context.graphics.paper_area.w);

        //Feed to the y height to ensure we catch any cut advances
        self.paper_image
            .expand_to_height(context.graphics.render_area.y);

        let rendered = self.paper_image.copy();

        ReceiptImage {
            width: rendered.0,
            height: rendered.1,
            bytes: rendered.2,
        }
    }
}
