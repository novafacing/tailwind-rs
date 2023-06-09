use super::*;

#[test]
fn test_typography_trace() {
    let (mut config, mut builder) = pre_config();
    config.mode = CssInlineMode::None;
    let (html, css) = config.compile_html(include_str!("typography.html"), &mut builder).unwrap();
    std::fs::write("tests/html/typography/typography.traced.html", html.as_bytes()).unwrap();
    std::fs::write("tests/html/typography/typography.traced.css", css.as_bytes()).unwrap();
    assert_eq!(html, include_str!("typography.traced.html"));
    assert_eq!(css, include_str!("typography.traced.css"));
}

#[test]
fn test_typography_inline() {
    let (mut config, mut builder) = pre_config();
    config.mode = CssInlineMode::Inline;
    let (html, css) = config.compile_html(include_str!("typography.html"), &mut builder).unwrap();
    std::fs::write("tests/html/typography/typography.inline.html", html.as_bytes()).unwrap();
    std::fs::write("tests/html/typography/typography.inline.css", css.as_bytes()).unwrap();
    assert_eq!(html, include_str!("typography.inline.html"));
    assert_eq!(css, include_str!("typography.inline.css"));
}
