use crate::widgets::footer;
use crate::widgets::head;
use crate::widgets::nav;

pub fn page() -> String {
    let head = head::html(false);
    let nav = nav::html(false);
    let footer = footer::html(false);

    format!(
        r#"
{head}
{nav}
<h2 id="become-a-member-today">Become a member today</h2>
{footer}
"#
    )
}
