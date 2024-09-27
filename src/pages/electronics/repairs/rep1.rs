use crate::widgets::footer;
use crate::widgets::head;
use crate::widgets::nav;

pub fn page() -> String {
    let styles = r#"
  <style>
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

  </style>
  <link rel="stylesheet" href="../../style.css" />
  <style>
    img{
	width: 50% !important;
	text-align: center;
	float: none !important;
    }
  </style>
"#;
    let footer = footer::html(false);

    format!(
        r#"
<!DOCTYPE html>
<html xmlns="http://www.w3.org/1999/xhtml" lang="" xml:lang="">
<head>
  <meta charset="utf-8" />
  <meta name="generator" content="pandoc" />
  <meta name="viewport" content="width=device-width, initial-scale=1.0, user-scalable=yes" />
  <meta name="author" content="Author xorokongo https://x.com/xorokongo" />
  <title>KHS | Xbox 360, E68 Error</title>
  {styles}
  <!--[if lt IE 9]>
    <script src="//cdnjs.cloudflare.com/ajax/libs/html5shiv/3.7.3/html5shiv-printshiv.min.js"></script>
  <![endif]-->
</head>
<body>
<header id="title-block-header">
  <h1 class="title">Xbox 360, E68 Error</h1>
  <img src="img/1.jpg">
<p class="author">Author seestem <a href="https://x.com/_seestem"
class="uri">https://x.com/_seestem</a></p>
<p class="date">Date 16.12.2023</p>
</header>
<hr />
<blockquote>
<p>Topics: <a href="../../topics/microsoft.html">#Microsoft</a>, <a
href="../../topics/xbox360.html">#Xbox360</a>, <a
href="../../topics/hardware.html">#Hardware</a></p>
</blockquote>
<h2 id="troubleshooting">Troubleshooting</h2>
<p>
  In our case the hard drive died and we had to replace the hard-drive
with a new one, below we give detailed steps how to replace a hard-drive
  for the Xbox.
</p>
<h2 id="the-fix">The Fix</h2>
<h3 id="replacing-the-hard-drive">Replacing the hard drive</h3>
<p>Just replace the hard drive, if you don't know how to do that follow this ifixit teardown:</p>
<p>
  <a href="https://www.ifixit.com/Teardown/Xbox+360+E+Teardown/15062#s49117"
     class="uri">https://www.ifixit.com/Teardown/Xbox+360+E+Teardown/15062#s49117</a>
</p>
<p>
  After the drives have been swaped your xbox should be working again.
</p>

<h2 id="references">References</h2>
<ul class="incremental">
  <li>
    <a href="https://www.ifixit.com/Teardown/Xbox+360+E+Teardown/15062#s49117">Xbox 360 E Teardown</a>
    <a href="https://support.xbox.com/en-US/help/errors/xbox-360/error-code-e68">Xbox Error code __E68__</a>
  </li>
</ul>
{footer}
</body>
</html>
"#
    )
}
