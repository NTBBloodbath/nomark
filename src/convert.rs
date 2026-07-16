use pulldown_cmark::{CodeBlockKind, Event, Options, Parser, Tag, TagEnd};

pub(crate) struct Converter {
    output: String,
    in_code_block: bool,
    code_block_lang: String,
    code_block_content: String,
    blockquote_depth: usize,
    list_stack: Vec<bool>,
    in_list_item: bool,
    in_footnote: bool,
    footnote_name: String,
    footnote_content: String,
    in_image: bool,
    image_url: String,
    image_alt: String,
    in_link: bool,
    in_metadata: bool,
    metadata_content: String,
    in_table: bool,
    table_alignments: Vec<pulldown_cmark::Alignment>,
    in_table_cell: bool,
    table_cur_cell_text: String,
    table_cur_row: Vec<String>,
    table_rows: Vec<Vec<String>>,
}

impl Converter {
    pub(crate) fn new() -> Self {
        Self {
            output: String::new(),
            in_code_block: false,
            code_block_lang: String::new(),
            code_block_content: String::new(),
            blockquote_depth: 0,
            list_stack: Vec::new(),
            in_list_item: false,
            in_footnote: false,
            footnote_name: String::new(),
            footnote_content: String::new(),
            in_image: false,
            image_url: String::new(),
            image_alt: String::new(),
            in_link: false,
            in_metadata: false,
            metadata_content: String::new(),
            in_table: false,
            table_alignments: Vec::new(),
            in_table_cell: false,
            table_cur_cell_text: String::new(),
            table_cur_row: Vec::new(),
            table_rows: Vec::new(),
        }
    }

    fn buf(&mut self, s: &str) {
        if self.in_footnote {
            self.footnote_content.push_str(s);
        } else if self.in_table_cell {
            self.table_cur_cell_text.push_str(s);
        } else {
            self.output.push_str(s);
        }
    }

    fn buf_char(&mut self, c: char) {
        if self.in_footnote {
            self.footnote_content.push(c);
        } else if self.in_table_cell {
            self.table_cur_cell_text.push(c);
        } else {
            self.output.push(c);
        }
    }

    pub(crate) fn convert(&mut self, input: &str) -> &str {
        let opts = Options::ENABLE_STRIKETHROUGH
            | Options::ENABLE_TABLES
            | Options::ENABLE_FOOTNOTES
            | Options::ENABLE_TASKLISTS
            | Options::ENABLE_YAML_STYLE_METADATA_BLOCKS
            | Options::ENABLE_PLUSES_DELIMITED_METADATA_BLOCKS
            | Options::ENABLE_OLD_FOOTNOTES
            | Options::ENABLE_GFM;

        let parser = Parser::new_ext(input, opts);
        for event in parser {
            match event {
                Event::Start(tag) => self.handle_start(tag),
                Event::End(tag) => self.handle_end(tag),
                Event::Text(text) => {
                    if self.in_code_block {
                        self.code_block_content.push_str(&text);
                    } else if self.in_metadata {
                        self.metadata_content.push_str(&text);
                    } else if self.in_image {
                        self.image_alt.push_str(&text);
                    } else {
                        self.buf(&escape_text(&text));
                    }
                }
                Event::Code(text) => {
                    self.buf_char('`');
                    self.buf(&text);
                    self.buf_char('`');
                }
                Event::Rule => self.buf("___\n\n"),
                Event::SoftBreak => self.buf_char(' '),
                Event::HardBreak => self.buf_char('\n'),
                Event::TaskListMarker(checked) => {
                    self.buf(if checked { "(x) " } else { "( ) " });
                }
                Event::FootnoteReference(name) => {
                    self.buf_char('^');
                    self.buf(&name);
                }
                Event::Html(html) => {
                    let trimmed = html.trim();
                    if !trimmed.is_empty() {
                        self.buf("@embed html\n");
                        self.buf(trimmed);
                        if !trimmed.ends_with('\n') {
                            self.buf_char('\n');
                        }
                        self.buf("@end\n\n");
                    }
                }
                Event::InlineHtml(html) => {
                    let text = strip_html(&html);
                    if !text.is_empty() {
                        self.buf(&text);
                    }
                }
                _ => {}
            }
        }
        self.output.trim_end().into()
    }

    fn handle_start(&mut self, tag: Tag<'_>) {
        match tag {
            Tag::Heading { level, .. } => {
                for _ in 0..level as u8 {
                    self.buf_char('*');
                }
                self.buf_char(' ');
            }
            Tag::CodeBlock(kind) => {
                self.in_code_block = true;
                self.code_block_lang = match kind {
                    CodeBlockKind::Fenced(lang) => lang.to_string(),
                    CodeBlockKind::Indented => String::new(),
                };
                self.code_block_content.clear();
            }
            Tag::BlockQuote(_) => self.blockquote_depth += 1,
            Tag::List(kind) => {
                if !self.list_stack.is_empty() {
                    self.buf_char('\n');
                }
                if self.list_stack.len() < 7 {
                    self.list_stack.push(kind.is_some());
                }
            }
            Tag::Item => {
                self.in_list_item = true;
                let depth = self.list_stack.len().max(1);
                let ch = if *self.list_stack.last().unwrap_or(&false) { '~' } else { '-' };
                for _ in 0..depth {
                    self.buf_char(ch);
                }
                self.buf_char(' ');
            }
            Tag::Paragraph => {
                for _ in 0..self.blockquote_depth {
                    self.buf_char('>');
                }
                if self.blockquote_depth > 0 {
                    self.buf_char(' ');
                }
            }
            Tag::Emphasis => self.buf_char('/'),
            Tag::Strong => self.buf_char('*'),
            Tag::Strikethrough => self.buf_char('-'),
            Tag::Link { dest_url, link_type, .. } => {
                self.in_link = true;
                match link_type {
                    pulldown_cmark::LinkType::Autolink | pulldown_cmark::LinkType::Email => {
                        self.buf_char('{');
                        self.buf(dest_url.as_ref());
                        self.buf("}[");
                    }
                    _ => {
                        let url = dest_url.as_ref();
                        if url.is_empty() {
                            self.buf("{}[");
                        } else if is_internal_path(url) {
                            self.buf("{:");
                            self.buf(url);
                            self.buf(":}[");
                        } else {
                            self.buf_char('{');
                            self.buf(url);
                            self.buf("}[");
                        }
                    }
                }
            }
            Tag::Image { dest_url, .. } => {
                self.in_image = true;
                self.image_url = dest_url.to_string();
                self.image_alt.clear();
            }
            Tag::FootnoteDefinition(name) => {
                self.in_footnote = true;
                self.footnote_name = name.to_string();
            }
            Tag::MetadataBlock(_) => {
                self.in_metadata = true;
                self.metadata_content.clear();
            }
            Tag::Table(alignments) => {
                self.in_table = true;
                self.table_alignments = alignments.clone();
                self.table_rows.clear();
            }
            Tag::TableRow => {
                self.table_cur_row.clear();
            }
            Tag::TableCell => {
                self.in_table_cell = true;
                self.table_cur_cell_text.clear();
            }
            _ => {}
        }
    }

    fn handle_end(&mut self, tag: TagEnd) {
        match tag {
            TagEnd::Heading { .. } => self.buf("\n\n"),
            TagEnd::CodeBlock => {
                self.in_code_block = false;
                let lang = self.code_block_lang.clone();
                let content = self.code_block_content.trim_end_matches('\n').to_string();
                self.buf("@code ");
                self.buf(&lang);
                self.buf_char('\n');
                self.buf(&content);
                self.buf("\n@end\n\n");
            }
            TagEnd::BlockQuote(_) => {
                self.blockquote_depth = self.blockquote_depth.saturating_sub(1);
            }
            TagEnd::Paragraph => {
                if !self.in_list_item {
                    self.buf("\n\n");
                }
            }
            TagEnd::Emphasis => self.buf_char('/'),
            TagEnd::Strong => self.buf_char('*'),
            TagEnd::Strikethrough => self.buf_char('-'),
            TagEnd::Link => {
                self.in_link = false;
                self.buf_char(']');
            }
            TagEnd::Image => {
                self.in_image = false;
                let url = std::mem::take(&mut self.image_url);
                let alt = std::mem::take(&mut self.image_alt);
                self.buf(".image ");
                self.buf(&url);
                if !alt.is_empty() {
                    self.buf(" ");
                    self.buf(&alt);
                }
                if !self.in_link {
                    self.buf_char('\n');
                }
            }
            TagEnd::List(_) => {
                self.list_stack.pop();
                if self.list_stack.is_empty() {
                    self.buf_char('\n');
                }
            }
            TagEnd::Item => {
                self.in_list_item = false;
                self.buf_char('\n');
            }
            TagEnd::FootnoteDefinition => {
                self.in_footnote = false;
                let name = std::mem::take(&mut self.footnote_name);
                let content = std::mem::take(&mut self.footnote_content);
                let trimmed = content.trim();
                self.buf("^^ ");
                self.buf(&name);
                self.buf_char('\n');
                self.buf(trimmed);
                self.buf("\n^^\n\n");
            }
            TagEnd::MetadataBlock(_) => {
                self.in_metadata = false;
                let content = std::mem::take(&mut self.metadata_content);
                let trimmed = content.trim();
                self.buf("@document.meta");
                self.buf_char('\n');
                if !trimmed.is_empty() {
                    let mut meta = String::new();
                    yaml_to_norg(trimmed, &mut meta);
                    self.buf(&meta);
                }
                self.buf("@end");
                self.buf_char('\n');
                self.buf_char('\n');
            }
            TagEnd::TableHead => {
                self.table_rows.push(std::mem::take(&mut self.table_cur_row));
                let align = std::mem::take(&mut self.table_alignments);
                let mut sep = Vec::new();
                for a in &align {
                    match a {
                        pulldown_cmark::Alignment::None => sep.push("---".to_string()),
                        pulldown_cmark::Alignment::Left => sep.push(":---".to_string()),
                        pulldown_cmark::Alignment::Center => sep.push(":---:".to_string()),
                        pulldown_cmark::Alignment::Right => sep.push("---:".to_string()),
                    }
                }
                if !sep.is_empty() {
                    self.table_rows.push(sep);
                }
            }
            TagEnd::TableCell => {
                self.in_table_cell = false;
                let text = std::mem::take(&mut self.table_cur_cell_text);
                self.table_cur_row.push(text.trim().to_string());
            }
            TagEnd::TableRow => {
                self.table_rows.push(std::mem::take(&mut self.table_cur_row));
            }
            TagEnd::Table => {
                self.in_table = false;
                let rows = std::mem::take(&mut self.table_rows);
                self.buf("@table\n");
                for row in &rows {
                    self.buf("| ");
                    self.buf(&row.join(" | "));
                    self.buf(" |\n");
                }
                self.buf("@end\n\n");
            }
            _ => {}
        }
    }
}

fn strip_html(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut in_tag = false;
    let mut tag_buf = String::new();
    for c in s.chars() {
        match c {
            '<' => {
                in_tag = true;
                tag_buf.clear();
                tag_buf.push(c);
            }
            '>' => {
                in_tag = false;
                tag_buf.clear();
            }
            _ if in_tag => {
                tag_buf.push(c);
            }
            _ => out.push(c),
        }
    }
    out
}

fn escape_text(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '\\' => out.push_str("\\\\"),
            '{' => out.push_str("\\{"),
            '}' => out.push_str("\\}"),
            '[' => out.push_str("\\["),
            ']' => out.push_str("\\]"),
            _ => out.push(c),
        }
    }
    out
}

fn is_internal_path(url: &str) -> bool {
    !(url.contains("://")
        || url.starts_with("mailto:")
        || url.starts_with("tel:")
        || url.starts_with("data:")
        || url.starts_with("javascript:"))
}

fn yaml_to_norg(yaml: &str, buf: &mut String) {
    if let Ok(value) = serde_yaml::from_str::<serde_yaml::Value>(yaml) {
        if let serde_yaml::Value::Mapping(map) = value {
            for (key, val) in &map {
                let k = key.as_str().unwrap_or("");
                buf.push_str(k);
                buf.push_str(": ");
                match val {
                    serde_yaml::Value::Sequence(seq) => {
                        buf.push_str("[\n");
                        for item in seq {
                            buf.push_str("  ");
                            buf.push_str(&norg_value(item));
                            buf.push('\n');
                        }
                        buf.push(']');
                    }
                    other => buf.push_str(&norg_value(other)),
                }
                buf.push('\n');
            }
            return;
        }
    }
    buf.push_str(yaml);
    buf.push('\n');
}

fn norg_value(v: &serde_yaml::Value) -> String {
    match v {
        serde_yaml::Value::String(s) => s.clone(),
        serde_yaml::Value::Number(n) => n.to_string(),
        serde_yaml::Value::Bool(b) => b.to_string(),
        serde_yaml::Value::Null => "nil".into(),
        serde_yaml::Value::Sequence(seq) => {
            let mut out = String::from("[\n");
            for item in seq {
                out.push_str("  ");
                out.push_str(&norg_value(item));
                out.push('\n');
            }
            out.push(']');
            out
        }
        other => serde_yaml::to_string(other).unwrap_or_default().trim().to_string(),
    }
}

pub(crate) fn convert_markdown(input: &str) -> String {
    let mut conv = Converter::new();
    conv.convert(input).to_string()
}
