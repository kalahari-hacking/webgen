use crate::widgets::footer;
use crate::widgets::head;
use crate::widgets::nav;

pub fn page() -> String {
    let footer = footer::html(false);
    let styles = r#"  <style>
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
  </style>"#;

    format!(
        r#"
<!DOCTYPE html>
<html xmlns="http://www.w3.org/1999/xhtml" lang="" xml:lang="">
<head>
  <meta charset="utf-8" />
  <meta name="generator" content="pandoc" />
  <meta name="viewport" content="width=device-width, initial-scale=1.0, user-scalable=yes" />
  <meta name="author" content="Author seestem https://x.com/_seestem" />
  <title>KHS | PS3 super slim green light but no video</title>
  {styles}
  <!--[if lt IE 9]>
    <script src="//cdnjs.cloudflare.com/ajax/libs/html5shiv/3.7.3/html5shiv-printshiv.min.js"></script>
  <![endif]-->
</head>
<body>
<header id="title-block-header">
  <h1 class="title">PS3 super slim green light but no video</h1>
  <img src="img/0.jpg">
<p class="author">Author seestem <a href="https://x.com/_seestem"
class="uri">https://x.com/_seestem</a></p>
<p class="date">Date 15.12.2023</p>
</header>
<nav id="TOC" role="doc-toc">
<ul class="incremental">
<li><a href="\#troubleshooting"
id="toc-troubleshooting">Troubleshooting</a></li>
<li><a href="\#the-fix" id="toc-the-fix">The Fix</a>
<ul class="incremental">
<li><a href="\#download-ps3-firmware"
id="toc-download-ps3-firmware">Download PS3 Firmware</a></li>
<li><a href="\#replacing-the-hard-drive"
id="toc-replacing-the-hard-drive">Replacing the hard drive</a></li>
</ul></li>
<li><a href="\#references" id="toc-references">References</a></li>
</ul>
</nav>
<hr />
<blockquote>
<p>Topics: <a href="../../topics/sony.html">#Sony</a>, <a
href="../../topics/ps3.html">#PS3</a>, <a
href="../../topics/hardware.html">#Hardware</a>, <a
href="../../topics/software.html">#Software</a></p>
</blockquote>
<h2 id="troubleshooting">Troubleshooting</h2>
<p>In our case the hard drive died and we had to replace the hard-drive
with a new one, below we give detailed steps how to replace a hard-drive
for all PS3 models (not just the super slim).</p>
<h2 id="the-fix">The Fix</h2>
<h3 id="download-ps3-firmware">Download PS3 Firmware</h3>
<p>First we will need to download the PS3 firmware to install on the new
hard drive. To do that follow these <strong>4</strong> steps:</p>
<ol class="incremental" type="1">
<li>Get a usb flash drive and format it a FAT32</li>
<li>In the USB flash drive Create a new folder with the name
<strong>PS3</strong> (all caps)</li>
<li>In the newly created PS3 folder, create another folder with the name
<strong>UPDATE</strong> (all caps)</li>
<li>Download PS3 firmware: <a
href="https://www.playstation.com/en-us/support/hardware/ps3/system-software/"
class="uri">https://www.playstation.com/en-us/support/hardware/ps3/system-software/</a>
and place it in the <strong>PS3/UPDATE/</strong> folder you just created
in the flash drive.</li>
</ol>
<h3 id="replacing-the-hard-drive">Replacing the hard drive</h3>
<p>Get a new replacement hard drive and just like the usb flash drive,
also format it as FAT32.</p>
<p>Replacing the drive is quite easy to do, just remove the right side
panel (Side of the LED) using a plastic card to scrape it open. Than
unscrew the blue screw and pull out the faulty hard drive. Or follow
this ifixit teardown to see how to swap the drive:</p>
<p><a
href="https://www.ifixit.com/Teardown/PlayStation+3+Super+Slim+Teardown/10670#s38613"
class="uri">https://www.ifixit.com/Teardown/PlayStation+3+Super+Slim+Teardown/10670#s38613</a></p>
<p>After the drives have been swaped follow these steps:</p>
<ol class="incremental" type="1">
<li>Switch on Device, you will see a message asking you to connect the
usb controller.</li>
<li>Connect controller and press <strong>PS</strong> button, you will
see a message that the system software cannot be run correctly.</li>
<li>Connect usb flash drive to PS3</li>
<li>Press <strong>START</strong> and <strong>SELECT</strong> buttons at
the same time</li>
<li>You see a message: <strong>Checking… Please wait</strong>.</li>
<li>A screen will appear, telling you that the drive will be
formated</li>
<li>Press <strong>START</strong> and <strong>SELECT</strong> buttons at
the same time again.</li>
<li>You see a message: <strong>Checking… Please wait</strong>.</li>
<li>After that it will proceed with the installation, follow the
  instructions and your PS3 will be working again</li>
</ol>
<h2 id="references">References</h2>
<ul class="incremental">
<li><a
href="https://www.ifixit.com/Teardown/PlayStation+3+Super+Slim+Teardown/10670">PlayStation
3 Super Slim Teardown</a></li>
<li><a href="https://www.psdevwiki.com/ps3/Boot_modes#Solid_Green">PS3
Boot modes</a></li>
<li><a
href="https://www.playstation.com/en-us/support/hardware/ps3/system-software/">PS3
Firmware</a></li>
</ul>
{footer}
</body>
</html>
"#
    )
}
