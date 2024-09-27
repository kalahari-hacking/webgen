use crate::widgets::footer;
use crate::widgets::head;
use crate::widgets::nav;

pub fn page() -> String {
    let head = head::html(true);
    let nav = nav::html(true);
    let footer = footer::html(true);

    format!(
        r#"
{head}
{nav}
<h2>⚡ electronics</h2>

<ul>
  <li><a href="./repairs.html">🪛 Repairs</a>
</ul>
{footer}
"#
    )
}
