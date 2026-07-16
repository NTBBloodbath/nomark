#[test]
fn link_simple() {
    let result = nomark::convert("[text](url)").unwrap();
    assert_eq!(result, "{url}[text]");
}

#[test]
fn link_with_bold() {
    let result = nomark::convert("[**bold**](url)").unwrap();
    assert_eq!(result, "{url}[*bold*]");
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
