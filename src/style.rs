use crate::config::Config;

pub fn generate_css(config: &Config) -> String {
    let ff_list = config.font_family.join(", ");

    format!(
        ".glint-window {{
            background-color: {bg}; 
            background-image: none; 
            border-radius: {b_radius}px; 
            border: {b_width}px solid {b_color};
            min-width: 300px;
        }} 
        label {{ 
            color: {text}; 
            padding: 25px 30px; 
            font-family: {ff};
            font-size: {size}pt;
            margin: 0;
        }}",
        ff = ff_list,
        bg = config.background_color,
        b_width = config.border_width,
        b_color = config.border_color,
        b_radius = config.border_radius,
        text = config.text_color,
        size = config.font_size,
    )
}
