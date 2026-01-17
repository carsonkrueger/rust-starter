use strum::IntoStaticStr;
use templr::{templ, templ_ret};
use tw_merge::tw_merge;

#[derive(Default, Copy, Clone)]
pub struct InputProps<'a> {
    pub id: Option<&'a str>,
    pub class: &'a str,
    pub input_type: InputType,
    pub attrs: &'a [(&'a str, &'a str)],
    pub name: &'a str,
    pub value: &'a str,
}

#[derive(IntoStaticStr, Default, Copy, Clone)]
pub enum InputType {
    #[default]
    Text,
    Password,
    Email,
    Number,
}

impl ToString for InputType {
    fn to_string(&self) -> String {
        let str: &str = self.into();
        str.to_string()
    }
}

pub fn input<'a>(props: InputProps<'a>) -> templ_ret!['a] {
    templ! {
        <input
            #if let Some(id) = props.id {
                id={id}
            }
            type={props.input_type.to_string()}
            name={props.name}
            value={props.value}
            class={tw_merge!(
                // Base styles
                "flex h-9 w-full min-w-0 rounded-md border border-input bg-transparent/30 px-3 py-1 text-base shadow-xs transition-[color,box-shadow] outline-none md:text-sm",
                // Dark mode background
                "dark:bg-input/30",
                // Selection styles
                "selection:bg-primary selection:text-primary-foreground",
                // Placeholder
                "placeholder:text-muted-foreground",
                // File input styles
                "file:inline-flex file:h-7 file:border-0 file:bg-transparent file:text-sm file:font-medium file:text-foreground",
                // Focus styles
                "focus-visible:border-ring focus-visible:ring-ring/50 focus-visible:ring-[3px]",
                // Disabled styles
                "disabled:pointer-events-none disabled:cursor-not-allowed disabled:opacity-50",
                // Error/Invalid styles
                "aria-invalid:ring-destructive/20 aria-invalid:border-destructive dark:aria-invalid:ring-destructive/40",
                props.class)}
            {..props.attrs}
        />
    }
}
