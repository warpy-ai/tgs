use colored::Colorize;
use colored::*;
use sysinfo::{System, SystemExt};

fn display_neon_art_with_gradient() {
    let ascii_art = r#"
********************************************************************
********************************************************************
********************************************************************
**                ,----------------,              ,---------,     **
**           ,-----------------------,          ,"        ,"|     **
**         ,"                      ,"|        ,"        ,"  |     **
**        +-----------------------+  |      ,"        ,"    |     **
**        |  .-----------------.  |  |     +---------+      |     **
**        |  |                 |  |  |     | -==----'|      |     **
**        |  |  W a r p y .    |  |  |     |         |      |     **
**        |  |  Bad command or |  |  |/----|`---=    |      |     **
**        |  |  tgs:\>_        |  |  |   ,/|==== ooo |      ;     **
**        |  |                 |  |  |  // |(((( [33]|    ,"      **
**        |  `-----------------'  |," .;'| |((((     |  ,"        **
**        +-----------------------+  ;;  | |         |,"          **
**            /_)______________(_/  //'   | +---------+           **
**    ___________________________/___  `,                         **
**    /  oooooooooooooooo  .o.  oooo /,   \,"-----------          **
**    / ==ooooooooooooooo==.o.  ooo= //   ,`\--{)B     ,"         **
**    /_==__==========__==_ooo__ooo=_/'   /___________,"          **
**    `-----------------------------'                             **
********************************************************************
********************************************************************
********************************************************************


====================================================================
    "#;

    // Define gradient colors
    let start_color = Color::TrueColor {
        r: 255,
        g: 0,
        b: 255,
    }; // Magenta
    let end_color = Color::TrueColor {
        r: 0,
        g: 255,
        b: 255,
    }; // Cyan

    // Split the art into lines
    let lines: Vec<&str> = ascii_art.lines().collect();

    // Apply gradient to each line
    for (i, line) in lines.iter().enumerate() {
        let gradient_ratio = i as f32 / lines.len() as f32;
        let blended_color = blend_color(start_color, end_color, gradient_ratio);
        println!("{}", line.color(blended_color));
    }
}

fn display_gradient_text(text: &str, start_color: Color, end_color: Color) {
    // Split the text into lines
    let lines: Vec<&str> = text.lines().collect();

    // Apply gradient to each line
    for (i, line) in lines.iter().enumerate() {
        let gradient_ratio = i as f32 / lines.len() as f32;
        let blended_color = blend_color(start_color, end_color, gradient_ratio);
        println!("{}", line.color(blended_color));
    }
}

fn blend_color(start: Color, end: Color, ratio: f32) -> Color {
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

pub fn display_welcome_message() {
    let mut system = System::new_all();
    system.refresh_all();

    display_neon_art_with_gradient();

    let welcome_text = format!(
        "Welcome to Terminal Generative Shell\nHostname: {}\nOS: {}\nUptime: {:?}\n",
        system.host_name().unwrap_or_default(),
        system.os_version().unwrap_or_default(),
        system.uptime()
    );

    // Define gradient colors for the welcome text
    let start_color_welcome = Color::TrueColor {
        r: 255,
        g: 165,
        b: 0, // Orange
    };
    let end_color_welcome = Color::TrueColor {
        r: 255,
        g: 0,
        b: 255, // Magenta
    };

    display_gradient_text(&welcome_text, start_color_welcome, end_color_welcome);
}
