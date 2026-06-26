pub(crate) fn write(mut raw: &str, mut to: String) -> String {
    while let Some(index) = raw.find(&['\'', '"', '&', '<', '>']) {
        to.push_str(&raw[..index]);
        to.push_str(match raw.as_bytes()[index] {
            b'\'' => "&apos;",
            b'"' => "&quot;",
            b'&' => "&amp;",
            b'<' => "&lt;",
            b'>' => "&gt;",
            _ => "",
        });
        raw = &raw[index + 1..];
    }

    to.push_str(raw);
    to
}
