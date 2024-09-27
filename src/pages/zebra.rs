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
<h3 id="zebra">🦓 zebra</h3>
<p>
  Zebra is a penetration testing program by the <b>Kalahari
  Hacking Society</b>, it is currently under active development,
  you can access the source code on
  <a href="https://github.com/kalahari-hacking/zebra">github</a>.
</p>
{footer}
"#
    )
}
