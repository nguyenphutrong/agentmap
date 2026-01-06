#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Priority {
    High,
    Medium,
    Low,
}

impl std::fmt::Display for Priority {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Priority::High => write!(f, "High"),
            Priority::Medium => write!(f, "Medium"),
            Priority::Low => write!(f, "Low"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MemoryKind {
    Warning,
    BusinessRule,
    Invariant,
    Todo,
    Fixme,
    Deprecated,
    Note,
    Hack,
    Safety,
}

impl MemoryKind {
    pub fn default_priority(&self) -> Priority {
        match self {
            MemoryKind::Warning | MemoryKind::Safety | MemoryKind::Deprecated => Priority::High,
            MemoryKind::BusinessRule | MemoryKind::Invariant => Priority::High,
            MemoryKind::Todo | MemoryKind::Fixme | MemoryKind::Hack => Priority::Medium,
            MemoryKind::Note => Priority::Low,
        }
    }

    pub fn category(&self) -> &'static str {
        match self {
            MemoryKind::Warning | MemoryKind::Safety => "Warnings",
            MemoryKind::BusinessRule | MemoryKind::Invariant => "Business Rules",
            MemoryKind::Todo | MemoryKind::Fixme | MemoryKind::Hack | MemoryKind::Deprecated => {
                "Technical Debt"
            }
            MemoryKind::Note => "Notes",
        }
    }

    pub fn emoji(&self) -> &'static str {
        match self {
            MemoryKind::Warning | MemoryKind::Safety => "⚠️",
            MemoryKind::BusinessRule | MemoryKind::Invariant => "📋",
            MemoryKind::Todo | MemoryKind::Fixme | MemoryKind::Hack | MemoryKind::Deprecated => {
                "🔧"
            }
            MemoryKind::Note => "📝",
        }
    }
}

impl std::fmt::Display for MemoryKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MemoryKind::Warning => write!(f, "WARNING"),
            MemoryKind::BusinessRule => write!(f, "RULE"),
            MemoryKind::Invariant => write!(f, "INVARIANT"),
            MemoryKind::Todo => write!(f, "TODO"),
            MemoryKind::Fixme => write!(f, "FIXME"),
            MemoryKind::Deprecated => write!(f, "DEPRECATED"),
            MemoryKind::Note => write!(f, "NOTE"),
            MemoryKind::Hack => write!(f, "HACK"),
            MemoryKind::Safety => write!(f, "SAFETY"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct MemoryEntry {
    pub kind: MemoryKind,
    pub content: String,
    pub source_file: String,
    pub line_number: usize,
    pub priority: Priority,
}

impl MemoryEntry {
    pub fn new(kind: MemoryKind, content: String, source_file: String, line_number: usize) -> Self {
        let priority = kind.default_priority();
        Self {
            kind,
            content,
            source_file,
            line_number,
            priority,
        }
    }

    pub fn with_priority(mut self, priority: Priority) -> Self {
        self.priority = priority;
        self
    }
}
