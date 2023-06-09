use super::*;

#[test]
fn test_effect_trace() {
    let (mut config, mut builder) = pre_config();
    config.mode = CssInlineMode::None;
    let (html, css) = config.compile_html(include_str!("effect.html"), &mut builder).unwrap();
    // std::fs::write("tests/html/effect/effect.traced.html", html.as_bytes()).unwrap();
    // std::fs::write("tests/html/effect/effect.traced.css", css.as_bytes()).unwrap();
    assert_eq!(html, include_str!("effect.traced.html"));
    assert_eq!(css, include_str!("effect.traced.css"));
}

#[test]
fn test_effect_inline() {
    let (mut config, mut builder) = pre_config();
    config.mode = CssInlineMode::Inline;
    let (html, css) = config.compile_html(include_str!("effect.html"), &mut builder).unwrap();
    // std::fs::write("tests/html/effect/effect.inline.html", html.as_bytes()).unwrap();
    // std::fs::write("tests/html/effect/effect.inline.css", css.as_bytes()).unwrap();
    assert_eq!(html, include_str!("effect.inline.html"));
    assert_eq!(css, include_str!("effect.inline.css"));
}
