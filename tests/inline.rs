#[test]
fn bold() {
    let result = nomark::convert("**bold**").unwrap();
    assert_eq!(result, "*bold*");
}

#[test]
fn italic_star() {
    let result = nomark::convert("*italic*").unwrap();
    assert_eq!(result, "/italic/");
}

#[test]
fn italic_underscore() {
    let result = nomark::convert("_italic_").unwrap();
    assert_eq!(result, "/italic/");
}

#[test]
fn code() {
    let result = nomark::convert("`code`").unwrap();
    assert_eq!(result, "`code`");
}

#[test]
fn strikethrough() {
    let result = nomark::convert("~~strike~~").unwrap();
    assert_eq!(result, "-strike-");
}

#[test]
fn nested_bold_italic() {
    // *** → Start(Emphasis), Start(Strong) in pulldown-cmark
    let result = nomark::convert("***both***").unwrap();
    assert_eq!(result, "/*both*/");
}

#[test]
fn bold_inside_italic() {
    let result = nomark::convert("**bold** and *italic*").unwrap();
    assert_eq!(result, "*bold* and /italic/");
}

#[test]
fn inline_mix() {
    let result = nomark::convert("**bold** and *italic* and `code`").unwrap();
    assert_eq!(result, "*bold* and /italic/ and `code`");
}
