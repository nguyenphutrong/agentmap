#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SymbolKind {
    Function,
    Method,
    Class,
    Struct,
    Enum,
    Trait,
    Interface,
    Const,
    Module,
    Type,
}

impl std::fmt::Display for SymbolKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SymbolKind::Function => write!(f, "fn"),
            SymbolKind::Method => write!(f, "method"),
            SymbolKind::Class => write!(f, "class"),
            SymbolKind::Struct => write!(f, "struct"),
            SymbolKind::Enum => write!(f, "enum"),
            SymbolKind::Trait => write!(f, "trait"),
            SymbolKind::Interface => write!(f, "interface"),
            SymbolKind::Const => write!(f, "const"),
            SymbolKind::Module => write!(f, "mod"),
            SymbolKind::Type => write!(f, "type"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Visibility {
    Public,
    Private,
    Protected,
    Internal,
}

impl std::fmt::Display for Visibility {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Visibility::Public => write!(f, "pub"),
            Visibility::Private => write!(f, "(private)"),
            Visibility::Protected => write!(f, "(protected)"),
            Visibility::Internal => write!(f, "(internal)"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LineRange {
    pub start: usize,
    pub end: usize,
}

impl LineRange {
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }

    pub fn single(line: usize) -> Self {
        Self {
            start: line,
            end: line,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Symbol {
    pub kind: SymbolKind,
    pub name: String,
    pub signature: Option<String>,
    pub line_range: LineRange,
    pub visibility: Visibility,
    pub doc_comment: Option<String>,
}

impl Symbol {
    pub fn new(kind: SymbolKind, name: String, line: usize, visibility: Visibility) -> Self {
        Self {
            kind,
            name,
            signature: None,
            line_range: LineRange::single(line),
            visibility,
            doc_comment: None,
        }
    }

    pub fn with_signature(mut self, signature: String) -> Self {
        self.signature = Some(signature);
        self
    }

    pub fn with_line_range(mut self, start: usize, end: usize) -> Self {
        self.line_range = LineRange::new(start, end);
        self
    }

    pub fn with_doc_comment(mut self, doc: String) -> Self {
        self.doc_comment = Some(doc);
        self
    }
}
