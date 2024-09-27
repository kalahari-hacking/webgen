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
    <h2 id="academy-syllabus">🎒 Academy Syllabus</h2>
    <ul class="incremental">
      <li><strong>Electronics</strong>
	<ul class="incremental">
	  <li>Ohms Law</li>
	  <li>Components</li>
	  <li>Circuits</li>
	  <li>Reverse Engineering</li>
      </ul></li>
      <li>Number of Systems
	<ul class="incremental">
	  <li>Decimal Numbers</li>
	  <li>Binary Numbers</li>
	  <li>Hexadecimal Numbers</li>
	  <li>Base64</li>
      </ul></li>
      <li>Analog &amp; Digital Data</li>
      <li>Antenna Theory &amp; Radio Frequencies</li>
      <li>OSI &amp; Networking Protocols</li>
      <li>Operating Systems
	<ul class="incremental">
	  <li>Web Security</li>
	  <li>Linux Security</li>
	  <li>Android Security</li>
	  <li>IOS / Mac Security</li>
      </ul></li>
      <li>Cryptography</li>
      <li>Business Operations Security</li>
      <li>Artificial Intelligence Security</li>
      <li>Quantum Computing</li>
    </ul>
{footer}
"#
    )
}
