use crate::{command_sets::CommandSet, commands::*};

//These should always be in alphabetical order
pub fn new() -> CommandSet {
    let commands = vec![
        barcode::new(),
        bit_image::new(),
        cancel::new(),
        carriage_return::new(),
        code_2d::new(),
        default_line_spacing::new(),
        feed_and_cut::new(),
        formfeed::new(),
        graphics::new(),
        horizontal_tab::new(),
        initialize::new(),
        large_graphics::new(),
        linefeed::new(),
        paper_end_sensor::new(),
        print_and_feed_lines::new(),
        print_and_feed::new(),
        print_and_reverse_feed_lines::new(),
        print_stop_sensor::new(),
        pulse::new(),
        raster_bit_image::new(),
        request_response_transmission::new(),
        set_absolute_print_pos::new(),
        set_alt_color::new(),
        set_barcode_height::new(),
        set_barcode_width::new(),
        set_black_white_invert::new(),
        set_character_size::new(),
        set_code_table::new(),
        set_double_strike::new(),
        set_emphasis::new(),
        set_font::new(),
        set_graphics_x_y::new(),
        set_barcode_font::new(),
        set_barcode_hri::new(),
        set_international_charset::new(),
        set_italic_off::new(), //NOT part of ESCPOS - not a truly valid esc/pos command but it doesn't conflict with anything in the spec
        set_italic_off_2::new(), //NOT part of ESCPOS
        set_italic_on::new(),  //NOT part of ESCPOS
        set_italic_on_2::new(), //NOT part of ESCPOS
        set_justification::new(),
        set_line_spacing::new(),
        set_panel_buttons::new(),
        set_peripheral_device::new(),
        set_print_mode::new(),
        set_relative_vertical_print::new(),
        set_smoothing::new(),
        set_tab_len::new(),
        set_underline::new(),
        set_upside_down::new(),
        transmit_printer_id::new(),
        set_page_mode::new(),
        set_absolute_vertical_print_position::new(),
        page_mode_print_area::new(),
        page_mode_print_direction::new(),
        page_mode_print_data::new(),
        select_standard_mode::new(),
    ];

    CommandSet {
        default: text::new(),
        unknown: unknown::new(),
        begin_parsing: begin_print::new(),
        end_parsing: end_print::new(),
        commands: Box::from(commands),
    }
}
