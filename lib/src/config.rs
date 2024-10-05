use formatter::config::FormattingConfig;
use serde::Deserialize;

#[derive(Debug, Clone, Copy, Deserialize)]
pub struct Config {
    pub indent: i32,
    pub line_length: i32,
    pub embracing_op_no_nl: bool,
    pub allow_nl_after_assignment: bool,
}

impl FormattingConfig for Config {
    fn line_length(&self) -> i32 {
        self.line_length
    }

    fn indent(&self) -> i32 {
        self.indent
    }

    fn embracing_op_no_nl(&self) -> bool {
        self.embracing_op_no_nl
    }

    fn allow_nl_after_assignment(&self) -> bool {
        self.allow_nl_after_assignment
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            indent: 2,
            line_length: 120,
            embracing_op_no_nl: true,
            allow_nl_after_assignment: false,
        }
    }
}

impl std::fmt::Display for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "indent: {} line_length: {} allow_nl_after_assignment: {}",
            self.indent, self.line_length, self.allow_nl_after_assignment
        ))
    }
}

impl Config {
    pub fn new(
        indent: i32,
        line_length: i32,
        embracing_op_no_nl: bool,
        allow_nl_after_assignment: bool,
    ) -> Self {
        Self {
            indent,
            line_length,
            embracing_op_no_nl,
            allow_nl_after_assignment,
        }
    }
}
