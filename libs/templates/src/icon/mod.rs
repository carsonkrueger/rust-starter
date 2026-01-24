use std::fmt::Write as _;

use templr::{FnTemplate, templ_ret};

use crate::icon::icon_data::Icon;

pub mod icon_data;

#[derive(Debug, Clone)]
pub struct IconProps<'a> {
    pub icon: Icon,
    pub size: u32,
    pub color: &'a str,
    pub fill: &'a str,
    pub stroke: &'a str,
    pub stroke_width: &'a str,
    pub class: &'a str,
}

impl Default for IconProps<'_> {
    fn default() -> Self {
        IconProps {
            icon: Icon::default(),
            size: 24,
            color: "#000",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "2",
            class: "",
        }
    }
}

/// Returns a function that produces a Template for an icon name
pub fn icon<'a>(props: IconProps<'a>) -> templ_ret!['a] {
    FnTemplate::new(move |w, _ctx, _| {
        let svg = generate_svg(&props)?;
        w.write_str(&svg)?;
        Ok(())
    })
}

fn generate_svg(props: &IconProps<'_>) -> Result<String, std::fmt::Error> {
    let content: &'static str = props.icon.into();

    let size = props.size;
    let fill = props.fill;
    let stroke = props.stroke;
    let stroke_width = props.stroke_width;
    let class = props.class;
    let color = props.color;

    let mut svg = String::new();
    write!(
        svg,
        r#"<svg xmlns="http://www.w3.org/2000/svg"
            width="{size}"
            height="{size}"
            viewBox="0 0 24 24"
            color="{color}"
            fill="{fill}"
            stroke="{stroke}"
            stroke-width="{stroke_width}"
            stroke-linecap="round"
            stroke-linejoin="round"
            class="{class}"
            data-lucide="icon">{content}</svg>"#
    )?;

    Ok(svg)
}
