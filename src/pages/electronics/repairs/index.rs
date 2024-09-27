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
    <h3 id="some-of-our-fixes">🪛 Electronics Repairs: </h3>
    <br>
    <br>
    <ol class="incremental" start="0" type="1">
      <li><a href="repairs/0.html">PS3 super slim green light but no
	  video</a>
	<span class="topic-tags">
	  <a class="topic-tags-links" href="../../topics/sony.html"><code>#Sony</code></a>
	  <a class="topic-tags-links" href="../../topics/ps3.html"><code>#PS3</code></a>
	  <a class="topic-tags-links" href="../../topics/hardware.html"><code>#Hardware</code></a>
	  <a class="topic-tags-links" href="../../topics/software.html"><code>#Software</code></a>
	</span>
      </li>
      <li><a href="repairs/1.html">Xbox 360, E68 Error</a>
	<span class="topic-tags">
	  <a class="topic-tags-links" href="../../topics/microsoft.html"><code>#Microsoft</code></a>
	  <a class="topic-tags-links" href="../../topics/xbox360.html"><code>#Xbox360</code></a>
	  <a class="topic-tags-links" href="../../topics/hardware.html"><code>#Hardware</code></a>
	</span>
      </li>
    </ol>
{footer}
"#
    )
}
