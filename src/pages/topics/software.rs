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
    <h2 id="topics">Topics::<code>Software</code></h2>
    <ol class="incremental" type="1">
      <li><a href="../electronics/repairs/0.html">PS3 super slim green light but no video</a></li>
    </ol>
{footer}
"#
    )
}
