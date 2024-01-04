use colored::Colorize;
use colored::*;

pub mod custom {
    use super::*;
    pub const CYAN: Color = Color::TrueColor {
        r: 0,
        g: 255,
        b: 255,
    };

    pub const MAGENTA: Color = Color::TrueColor {
        r: 255,
        g: 0,
        b: 255,
    };

    pub const ORANGE: Color = Color::TrueColor {
        r: 255,
        g: 165,
        b: 0,
    };
}

pub fn display_gradient_text(text: &str, start_color: Color, end_color: Color) {
    // Split the text into lines
    let lines: Vec<&str> = text.lines().collect();

    // Apply gradient to each line
    for (i, line) in lines.iter().enumerate() {
        let gradient_ratio = i as f32 / lines.len() as f32;
        let blended_color = blend_color(start_color, end_color, gradient_ratio);
        println!("{}", line.color(blended_color));
    }
}

pub fn display_color_text(text: &str, color: Color) {
    println!("{}", text.color(color));
}

pub fn blend_color(start: Color, end: Color, ratio: f32) -> Color {
    if let (
        Color::TrueColor {
            r: r1,
            g: g1,
            b: b1,
        },
        Color::TrueColor {
            r: r2,
            g: g2,
            b: b2,
        },
    ) = (start, end)
    {
        let blended_r = blend_value(r1, r2, ratio);
        let blended_g = blend_value(g1, g2, ratio);
        let blended_b = blend_value(b1, b2, ratio);
        Color::TrueColor {
            r: blended_r,
            g: blended_g,
            b: blended_b,
        }
    } else {
        // Fallback to a default color if not TrueColor
        Color::White
    }
}

fn blend_value(start: u8, end: u8, ratio: f32) -> u8 {
    ((start as f32 * (1.0 - ratio)) + (end as f32 * ratio)) as u8
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let text = "Hello, World!";
        let start_color = Color::Red;
        let end_color = Color::Blue;
        display_gradient_text(text, start_color, end_color);
        display_color_text(text, Color::Green);
        let blended_color = blend_color(start_color, end_color, 0.5);
        println!("{:?}", blended_color);
    }
}
