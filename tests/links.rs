#[test]
fn link_simple() {
    let result = nomark::convert("[text](https://example.com)").unwrap();
    assert_eq!(result, "{https://example.com}[text]");
}

#[test]
fn link_with_bold() {
    let result = nomark::convert("[**bold**](https://example.com)").unwrap();
    assert_eq!(result, "{https://example.com}[*bold*]");
}

#[test]
fn internal_file_link() {
    let result = nomark::convert("[text](/path/to/page)").unwrap();
    assert_eq!(result, "{:/path/to/page:}[text]");
}

#[test]
fn internal_link_relative() {
    let result = nomark::convert("[text](../other)").unwrap();
    assert_eq!(result, "{:../other:}[text]");
}

#[test]
fn image_simple() {
    let result = nomark::convert("![alt](img.png)").unwrap();
    assert_eq!(result, ".image img.png alt");
}

#[test]
fn image_no_alt() {
    let result = nomark::convert("![](img.png)").unwrap();
    assert_eq!(result, ".image img.png");
}

#[test]
fn footnote_reference() {
    let result = nomark::convert("text[^1]").unwrap();
    assert_eq!(result, "text^1");
}

#[test]
fn footnote_definition() {
    let result = nomark::convert("[^1]: content").unwrap();
    assert_eq!(result, "^^ 1\ncontent\n^^");
}

#[test]
fn footnote_full() {
    let result = nomark::convert("text[^1]\n\n[^1]: content").unwrap();
    assert_eq!(result, "text^1\n\n^^ 1\ncontent\n^^");
}
