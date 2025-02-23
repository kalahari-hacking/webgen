pub fn html(absolute: bool) -> String {
    let absolute = if absolute { "../" } else { "" };

    let css = r#"<style>
	 code{white-space: pre-wrap;}
	 span.smallcaps{font-variant: small-caps;}
	 div.columns{display: flex; gap: min(4vw, 1.5em);}
	 div.column{flex: auto; overflow-x: auto;}
	 div.hanging-indent{margin-left: 1.5em; text-indent: -1.5em;}
	 /* The extra [class] is a hack that increases specificity enough to
	    override a similar rule in reveal.js */
	 ul.task-list[class]{list-style: none;}
	 ul.task-list li input[type="checkbox"] {
	     font-size: inherit;
	     width: 0.8em;
	     margin: 0 0.8em 0.2em -1.6em;
	     vertical-align: middle;
	 }
	 .display.math{display: block; text-align: center; margin: 0.5rem auto;}
	</style>"#;

    format!(
        r#"<!DOCTYPE html>
<html xmlns="http://www.w3.org/1999/xhtml" lang="" xml:lang="">
    <head>
	<meta charset="utf-8" />
	<meta name="generator" content="pandoc" />
	<meta name="viewport" content="width=device-width, initial-scale=1.0, user-scalable=yes" />
	<title>Kalahari Hacking Society</title>
	<link rel="stylesheet" href="{absolute}style.css" />
        {css}
	<!--[if lt IE 9]>
	    <script src="//cdnjs.cloudflare.com/ajax/libs/html5shiv/3.7.3/html5shiv-printshiv.min.js"></script>
	<![endif]-->
    </head>
"#
    )
}
