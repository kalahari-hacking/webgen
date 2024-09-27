pub fn html(absolute: bool) -> String {
    let absolute = if absolute { "../" } else { "" };

    format!(
        r#"
<p><img src="{absolute}kalahari.png" title="kalahari" /></p>
	<p><a href="{absolute}index.html"><strong>Kalahari Hacking Society</strong></a> is
	    a cyber-security think-tank, that hacks into systems, in order to make
	    them secure. These systems include both software and hardware
	    systems.</p>
	<hr />
	<p>
	    Home |
	    <a href="{absolute}academy.html">🎒Academy</a> |
	    <a href="{absolute}blog.html">📰 Blog</a> |
	    <a href="{absolute}electronics/index.html">⚡electronics</a> |
	    <a href="{absolute}shop.html">🛍️Shop</a> |
	    <a href="{absolute}contact.html">🤙 Contact</a> |
	    <a href="{absolute}zebra.html">🦓zebra</a>
	</p>
	<hr />
"#
    )
}
