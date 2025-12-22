pub mod pages;

use templr::{templ, templ_ret};

pub fn index<'a>() -> templ_ret!['a] {
    templ! {
        #use children;
        <!DOCTYPE html>
        <html>
            <head>
                <title>Rust Starter</title>
                <meta charset="UTF-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1.0" />
                <link href="/public/css/index.css" rel="stylesheet" />
            </head>
            <body>
                #children;
            </body>
        </html>
    }
}
