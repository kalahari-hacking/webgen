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
    <h2 id="topics">Topics</h2>
    <ol class="incremental" type="1">
      <li><a href="sony.html">#Sony</a></li>
      <li><a href="ps3.html">#PS3</a></li>
      <li><a href="hardware.html">#Hardware</a></li>
      <li><a href="software.html">#Software</a></li>
      <li><a href="microsoft.html">#Microsoft</a></li>
      <li><a href="xbox360.html">#Xbox360</a></li>
    </ol>
{footer}
"#
    )
}
