pub fn html(absolute: bool) -> String {
    let absolute = if absolute { "../" } else { "" };

    format!(
        r#"
    <hr />
    <p>
      <a href="{absolute}topics/index.html">Topics</a>
      | Powered by <a href="https://kwatafana.systems/">kwatafana.systems</a>
    </p>
"#
    )
}
