use std::{collections::HashMap, fmt::Write as _, sync::RwLock};

use once_cell::sync::Lazy;
use templr::{FnTemplate, templ_ret};

use crate::icon::icon_data::Icon;

pub mod icon_data;

static CACHED_ICONS: Lazy<RwLock<HashMap<String, String>>> =
    Lazy::new(|| RwLock::new(HashMap::new()));

#[derive(Debug, Clone)]
pub struct IconProps<'a> {
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
pub fn icon<'a>(icon: Icon, props: IconProps<'a>) -> templ_ret!['a] {
    let name = icon as u8;
    let cache_key = format!(
        "{name}|s:{:?}|c:{:?}|f:{:?}|sk:{:?}|sw:{:?}|cl:{:?}",
        props.size, props.color, props.fill, props.stroke, props.stroke_width, props.class
    );

    FnTemplate::new(move |w, _ctx, _| {
        // Fast path: read lock
        if let Some(svg) = CACHED_ICONS.read().unwrap().get(&cache_key) {
            w.write_str(svg)?;
            return Ok(());
        }

        // Generate SVG
        let svg = generate_svg(icon, &props)?;

        // Cache result
        CACHED_ICONS
            .write()
            .unwrap()
            .insert(cache_key.clone(), svg.clone());

        w.write_str(&svg)?;
        Ok(())
    })
}

fn generate_svg(icon: Icon, props: &IconProps<'_>) -> Result<String, std::fmt::Error> {
    let content: &'static str = icon.into();

    let size = props.size;
    let fill = props.fill;
    let stroke = props.stroke;
    let stroke_width = props.stroke_width;
    let class = props.class;

    let mut svg = String::new();
    write!(
        svg,
        r#"<svg xmlns="http://www.w3.org/2000/svg"
            width="{size}"
            height="{size}"
            viewBox="0 0 24 24"
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
