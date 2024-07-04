use std::borrow::Cow::{self, Borrowed, Owned};

use rustyline::{highlight::{Highlighter, MatchingBracketHighlighter}, hint::{Hinter, HistoryHinter}, validate::{MatchingBracketValidator, ValidationResult, Validator}, CompletionType, Config, EditMode, OutputStreamType};
use rustyline_derive::{Completer, Helper};

#[derive(Helper,Completer)]
pub struct REPLHelper{
    pub validator:MatchingBracketValidator,
    pub colored_prompt:String,
    pub hinter:HistoryHinter,
    pub highlighter:MatchingBracketHighlighter,
}
impl REPLHelper {
    #[warn(dead_code)]
    pub fn new()->Self{
        REPLHelper{
            highlighter:MatchingBracketHighlighter::new(),
            hinter:HistoryHinter{},
            colored_prompt:"".to_owned(),
            validator:MatchingBracketValidator::new(),
        }
    }
}
impl Hinter for  REPLHelper {
    type Hint = String;
    fn hint(&self, line: &str, pos: usize, ctx: &rustyline::Context<'_>) -> Option<Self::Hint> {
        self.hinter.hint(line, pos, ctx)
    }
}
impl Validator for  REPLHelper {
    fn validate(&self, ctx: &mut rustyline::validate::ValidationContext) -> rustyline::Result<rustyline::validate::ValidationResult> {
        use ValidationResult::{Incomplete, /*Invalid,*/ Valid};
        let input = ctx.input();
        // let result = if !input.starts_with("SELECT") {
        //     Invalid(Some(" --< Expect: SELECT stmt".to_owned()))
        // } else 
        let result = if input.eq(".exit") {
            Valid(None)
        } else if !input.ends_with(';') {
            Incomplete
        } else {
            Valid(None)
        };
        Ok(result)
    }
    fn validate_while_typing(&self) -> bool {
        self.validator.validate_while_typing()
    }
}
impl Highlighter for  REPLHelper {
    fn highlight_prompt<'b, 's: 'b, 'p: 'b>(&'s self, prompt: &'p str, default: bool,) -> Cow<'b, str> {
        if default {
            Borrowed(&self.colored_prompt)
        } else {
            Borrowed(prompt)
        }
    }

    // Takes the hint and returns the highlighted version (with ANSI color).
    fn highlight_hint<'h>(&self, hint: &'h str) -> Cow<'h, str> {
        Owned("\x1b[1m".to_owned() + hint + "\x1b[m")
    }

    // Takes the currently edited line with the cursor position and returns the highlighted version (with ANSI color).
    fn highlight<'l>(&self, line: &'l str, pos: usize) -> Cow<'l, str> {
        self.highlighter.highlight(line, pos)
    }

    // Tells if line needs to be highlighted when a specific char is typed or when cursor is moved under a specific char.
    // Used to optimize refresh when a character is inserted or the cursor is moved.
    fn highlight_char(&self, line: &str, pos: usize) -> bool {
        self.highlighter.highlight_char(line, pos)
    }
}
#[warn(dead_code)]
pub fn get_config() -> Config {
    Config::builder()
        .history_ignore_space(true)
        .completion_type(CompletionType::List)
        .edit_mode(EditMode::Emacs)
        .output_stream(OutputStreamType::Stdout)
        .build()
}