use models::db::auth::user::User;
use templates::table::{
    RowProps, TBodyProps, THeadProps, TableProps, TdProps, ThProps,
    infinite_scroll::{self, IntersectRowProps},
    row, table, tbody, td, th, thead,
};
use templr::{templ, templ_ret};

pub fn user_management_table<'a>(data: &'a [User]) -> templ_ret!['a] {
    templ! {
        #table(&TableProps{
            id: Some("user_management"),
            ..Default::default()
        }) {
            #thead(&THeadProps::default()) {
                #row(&RowProps::default()) {
                    #th(&ThProps::default()) {
                        ID
                    }
                    #th(&ThProps::default()) {
                        Name
                    }
                    #th(&ThProps::default()) {
                        Role
                    }
                    #th(&ThProps::default()) {
                        Joined
                    }
                    #th(&ThProps::default());
                }
            }
            #tbody(&TBodyProps::default()) {
                #for d in data {
                    #user_row(d);
                }
                #infinite_scroll::intersect_row(&IntersectRowProps{
                    endpoint: "/management/users/rows",
                    ..Default::default()
                });
            }
        }
    }
}

pub fn user_row<'a>(user: &'a User) -> templ_ret!['a] {
    templ! {
        #row(&RowProps::default()) {
            #td(&TdProps::default()) {
                {user.id}
            }
            #td(&TdProps::default()) {
                {user.first_name} {user.last_name}
            }
            #td(&TdProps::default()) {
                {user.role_id}
            }
            #td(&TdProps::default()) {
                {user.created_at.unwrap_or_default().to_string()}
            }
            #td(&TdProps::default());
        }
    }
}
