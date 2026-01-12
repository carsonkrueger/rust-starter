use templr::{Trust, templ, templ_ret};

/// to parameter is trusted
pub fn redirect_replace<'a>(to: &'a str) -> templ_ret!['a] {
    templ! {
        <script>window.location.replace({Trust(format!("'{}'", to))})</script>
    }
}

/// to parameter is trusted
pub fn redirect_assign<'a>(to: &'a str) -> templ_ret!['a] {
    templ! {
        <script>window.location.assign({Trust(format!("'{}'", to))})</script>
    }
}
