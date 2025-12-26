use templr::{templ, templ_ret};
use tw_merge::tw_merge;

pub struct InputProps<'a> {
    pub id: &'a str,
    pub class: &'a str,
    pub input_class: &'a str,
    pub attrs: &'a [(&'a str, &'a str)],
    pub name: &'a str,
    pub value: &'a str,
}

impl Default for InputProps<'_> {
    fn default() -> Self {
        InputProps {
            id: "",
            class: "",
            input_class: "",
            attrs: &[],
            name: "",
            value: "",
        }
    }
}

pub fn input<'a>(props: InputProps<'a>) -> templ_ret!['a] {
    templ! {
        <input
            #if !props.id.is_empty() {
                id={props.id}
            }
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
                props.input_class)}
            {..props.attrs}
        />
    }
}
