// Generated by rust-peg. Do not edit.
#![allow(non_snake_case, unused)]
use karma::{Karma, KarmaChange, KarmaStyle};
use self::RuleResult::{Matched, Failed};
fn escape_default(s: &str) -> String {
    s.chars().flat_map(|c| c.escape_default()).collect()
}
fn char_range_at(s: &str, pos: usize) -> (char, usize) {
    let c = &s[pos..].chars().next().unwrap();
    let next_pos = pos + c.len_utf8();
    (*c, next_pos)
}
enum RuleResult<T> { Matched(usize, T), Failed, }
struct ParseState {
    max_err_pos: usize,
    expected: ::std::collections::HashSet<&'static str>,
}
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct ParseError {
    pub line: usize,
    pub column: usize,
    pub offset: usize,
    pub expected: ::std::collections::HashSet<&'static str>,
}
pub type ParseResult<T> = Result<T, ParseError>;
impl ::std::fmt::Display for ParseError {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter)
     -> ::std::result::Result<(), ::std::fmt::Error> {
        try!(write ! (
             fmt , "error at {}:{}: expected " , self . line , self . column
             ));
        if self.expected.len() == 1 {
            try!(write ! (
                 fmt , "`{}`" , escape_default (
                 self . expected . iter (  ) . next (  ) . unwrap (  ) ) ));
        } else {
            let mut iter = self.expected.iter();
            try!(write ! (
                 fmt , "one of `{}`" , escape_default (
                 iter . next (  ) . unwrap (  ) ) ));
            for elem in iter {
                try!(write ! ( fmt , ", `{}`" , escape_default ( elem ) ));
            }
        }
        Ok(())
    }
}
impl ::std::error::Error for ParseError {
    fn description(&self) -> &str { "parse error" }
}
impl ParseState {
    fn new() -> ParseState {
        ParseState{max_err_pos: 0,
                   expected: ::std::collections::HashSet::new(),}
    }
    fn mark_failure(&mut self, pos: usize, expected: &'static str)
     -> RuleResult<()> {
        if pos > self.max_err_pos {
            self.max_err_pos = pos;
            self.expected.clear();
        }
        if pos == self.max_err_pos { self.expected.insert(expected); }
        Failed
    }
}
fn slice_eq(input: &str, state: &mut ParseState, pos: usize, m: &'static str)
 -> RuleResult<()> {
    #![inline]
    #![allow(dead_code)]
    let l = m.len();
    if input.len() >= pos + l &&
           &input.as_bytes()[pos..pos + l] == m.as_bytes() {
        Matched(pos + l, ())
    } else { state.mark_failure(pos, m) }
}
fn slice_eq_case_insensitive(input: &str, state: &mut ParseState, pos: usize,
                             m: &'static str) -> RuleResult<()> {
    #![inline]
    #![allow(dead_code)]
    let mut used = 0usize;
    let mut input_iter = input[pos..].chars().flat_map(|x| x.to_uppercase());
    for m_char_upper in m.chars().flat_map(|x| x.to_uppercase()) {
        used += m_char_upper.len_utf8();
        let input_char_result = input_iter.next();
        if input_char_result.is_none() ||
               input_char_result.unwrap() != m_char_upper {
            return state.mark_failure(pos, m);
        }
    }
    Matched(pos + used, ())
}
fn any_char(input: &str, state: &mut ParseState, pos: usize)
 -> RuleResult<()> {
    #![inline]
    #![allow(dead_code)]
    if input.len() > pos {
        let (_, next) = char_range_at(input, pos);
        Matched(next, ())
    } else { state.mark_failure(pos, "<character>") }
}
fn pos_to_line(input: &str, pos: usize) -> (usize, usize) {
    let mut remaining = pos;
    let mut lineno: usize = 1;
    for line in input.lines() {
        let line_length = line.len() + 1;
        if remaining < line_length { return (lineno, remaining + 1); }
        remaining -= line_length;
        lineno += 1;
    }
    return (lineno, remaining + 1);
}
fn parse_line<'input>(input: &'input str, state: &mut ParseState, pos: usize)
 -> RuleResult<Vec<Karma>> {
    {
        let start_pos = pos;
        {
            let seq_res =
                {
                    let mut repeat_pos = pos;
                    let mut repeat_value = vec!();
                    loop  {
                        let pos = repeat_pos;
                        let step_res = parse_element(input, state, pos);
                        match step_res {
                            Matched(newpos, value) => {
                                repeat_pos = newpos;
                                repeat_value.push(value);
                            }
                            Failed => { break ; }
                        }
                    }
                    Matched(repeat_pos, repeat_value)
                };
            match seq_res {
                Matched(pos, es) => {
                    {
                        let match_str = &input[start_pos..pos];
                        Matched(pos,
                                {
                                    es.iter().filter_map(|&ref x|
                                                             if let &Some(ref k)
                                                                    = x {
                                                                 Some(k.clone())
                                                             } else {
                                                                 None
                                                             }).collect()
                                })
                    }
                }
                Failed => Failed,
            }
        }
    }
}
fn parse_element<'input>(input: &'input str, state: &mut ParseState,
                         pos: usize) -> RuleResult<Option<Karma>> {
    {
        let choice_res =
            {
                let start_pos = pos;
                {
                    let seq_res = parse_karmachange(input, state, pos);
                    match seq_res {
                        Matched(pos, k) => {
                            {
                                let match_str = &input[start_pos..pos];
                                Matched(pos, { Some(k) })
                            }
                        }
                        Failed => Failed,
                    }
                }
            };
        match choice_res {
            Matched(pos, value) => Matched(pos, value),
            Failed => {
                let start_pos = pos;
                {
                    let seq_res = any_char(input, state, pos);
                    match seq_res {
                        Matched(pos, _) => {
                            {
                                let match_str = &input[start_pos..pos];
                                Matched(pos, { None })
                            }
                        }
                        Failed => Failed,
                    }
                }
            }
        }
    }
}
fn parse_karmachange<'input>(input: &'input str, state: &mut ParseState,
                             pos: usize) -> RuleResult<Karma> {
    {
        let choice_res = parse_explicit_karma_change(input, state, pos);
        match choice_res {
            Matched(pos, value) => Matched(pos, value),
            Failed => parse_implicit_karma_change(input, state, pos),
        }
    }
}
fn parse_explicit_karma_change<'input>(input: &'input str,
                                       state: &mut ParseState, pos: usize)
 -> RuleResult<Karma> {
    {
        let choice_res = parse_karma_notice_change(input, state, pos);
        match choice_res {
            Matched(pos, value) => Matched(pos, value),
            Failed => parse_karma_silent_change(input, state, pos),
        }
    }
}
fn parse_karma_notice_change<'input>(input: &'input str,
                                     state: &mut ParseState, pos: usize)
 -> RuleResult<Karma> {
    {
        let start_pos = pos;
        {
            let seq_res = slice_eq(input, state, pos, "[");
            match seq_res {
                Matched(pos, _) => {
                    {
                        let seq_res = parse_notice_chars(input, state, pos);
                        match seq_res {
                            Matched(pos, c) => {
                                {
                                    let seq_res =
                                        slice_eq(input, state, pos, "]");
                                    match seq_res {
                                        Matched(pos, _) => {
                                            {
                                                let seq_res =
                                                    parse_modifier(input,
                                                                   state,
                                                                   pos);
                                                match seq_res {
                                                    Matched(pos, m) => {
                                                        {
                                                            let match_str =
                                                                &input[start_pos..pos];
                                                            Matched(pos,
                                                                    {
                                                                        Karma{term:
                                                                                  c,
                                                                              change:
                                                                                  m,
                                                                              style:
                                                                                  KarmaStyle::Notify,}
                                                                    })
                                                        }
                                                    }
                                                    Failed => Failed,
                                                }
                                            }
                                        }
                                        Failed => Failed,
                                    }
                                }
                            }
                            Failed => Failed,
                        }
                    }
                }
                Failed => Failed,
            }
        }
    }
}
fn parse_karma_silent_change<'input>(input: &'input str,
                                     state: &mut ParseState, pos: usize)
 -> RuleResult<Karma> {
    {
        let start_pos = pos;
        {
            let seq_res = slice_eq(input, state, pos, "(");
            match seq_res {
                Matched(pos, _) => {
                    {
                        let seq_res = parse_silent_chars(input, state, pos);
                        match seq_res {
                            Matched(pos, c) => {
                                {
                                    let seq_res =
                                        slice_eq(input, state, pos, ")");
                                    match seq_res {
                                        Matched(pos, _) => {
                                            {
                                                let seq_res =
                                                    parse_modifier(input,
                                                                   state,
                                                                   pos);
                                                match seq_res {
                                                    Matched(pos, m) => {
                                                        {
                                                            let match_str =
                                                                &input[start_pos..pos];
                                                            Matched(pos,
                                                                    {
                                                                        Karma{term:
                                                                                  c,
                                                                              change:
                                                                                  m,
                                                                              style:
                                                                                  KarmaStyle::Silent,}
                                                                    })
                                                        }
                                                    }
                                                    Failed => Failed,
                                                }
                                            }
                                        }
                                        Failed => Failed,
                                    }
                                }
                            }
                            Failed => Failed,
                        }
                    }
                }
                Failed => Failed,
            }
        }
    }
}
fn parse_implicit_karma_change<'input>(input: &'input str,
                                       state: &mut ParseState, pos: usize)
 -> RuleResult<Karma> {
    {
        let start_pos = pos;
        {
            let seq_res = parse_implicit_chars(input, state, pos);
            match seq_res {
                Matched(pos, c) => {
                    {
                        let seq_res = parse_modifier(input, state, pos);
                        match seq_res {
                            Matched(pos, m) => {
                                {
                                    let match_str = &input[start_pos..pos];
                                    Matched(pos,
                                            {
                                                Karma{term: c,
                                                      change: m,
                                                      style:
                                                          KarmaStyle::Implicit,}
                                            })
                                }
                            }
                            Failed => Failed,
                        }
                    }
                }
                Failed => Failed,
            }
        }
    }
}
fn parse_implicit_chars<'input>(input: &'input str, state: &mut ParseState,
                                pos: usize) -> RuleResult<String> {
    {
        let start_pos = pos;
        {
            let seq_res =
                {
                    let mut repeat_pos = pos;
                    let mut repeat_value = vec!();
                    loop  {
                        let pos = repeat_pos;
                        let step_res = parse_implicit_char(input, state, pos);
                        match step_res {
                            Matched(newpos, value) => {
                                repeat_pos = newpos;
                                repeat_value.push(value);
                            }
                            Failed => { break ; }
                        }
                    }
                    if repeat_value.len() >= 1usize {
                        Matched(repeat_pos, repeat_value)
                    } else { Failed }
                };
            match seq_res {
                Matched(pos, cs) => {
                    {
                        let match_str = &input[start_pos..pos];
                        Matched(pos, { match_str.to_string() })
                    }
                }
                Failed => Failed,
            }
        }
    }
}
fn parse_implicit_char<'input>(input: &'input str, state: &mut ParseState,
                               pos: usize) -> RuleResult<String> {
    {
        let choice_res = parse_implicit_char_rest(input, state, pos);
        match choice_res {
            Matched(pos, value) => Matched(pos, value),
            Failed => {
                let start_pos = pos;
                {
                    let seq_res =
                        {
                            let seq_res = slice_eq(input, state, pos, "-");
                            match seq_res {
                                Matched(pos, _) => {
                                    {
                                        let seq_res =
                                            parse_implicit_char_rest(input,
                                                                     state,
                                                                     pos);
                                        match seq_res {
                                            Matched(pos, _) => {
                                                parse_implicit_chars(input,
                                                                     state,
                                                                     pos)
                                            }
                                            Failed => Failed,
                                        }
                                    }
                                }
                                Failed => Failed,
                            }
                        };
                    match seq_res {
                        Matched(pos, cs) => {
                            {
                                let match_str = &input[start_pos..pos];
                                Matched(pos, { match_str.to_string() })
                            }
                        }
                        Failed => Failed,
                    }
                }
            }
        }
    }
}
fn parse_implicit_char_rest<'input>(input: &'input str,
                                    state: &mut ParseState, pos: usize)
 -> RuleResult<String> {
    {
        let start_pos = pos;
        {
            let seq_res =
                if input.len() > pos {
                    let (ch, next) = char_range_at(input, pos);
                    match ch {
                        'a' ...'z' | 'A' ...'Z' | '0' ...'9' | '_' =>
                        Matched(next, ()),
                        _ => state.mark_failure(pos, "[a-zA-Z0-9_]"),
                    }
                } else { state.mark_failure(pos, "[a-zA-Z0-9_]") };
            match seq_res {
                Matched(pos, _) => {
                    {
                        let match_str = &input[start_pos..pos];
                        Matched(pos, { match_str.to_string() })
                    }
                }
                Failed => Failed,
            }
        }
    }
}
fn parse_notice_chars<'input>(input: &'input str, state: &mut ParseState,
                              pos: usize) -> RuleResult<String> {
    {
        let start_pos = pos;
        {
            let seq_res =
                {
                    let mut repeat_pos = pos;
                    let mut repeat_value = vec!();
                    loop  {
                        let pos = repeat_pos;
                        let step_res =
                            if input.len() > pos {
                                let (ch, next) = char_range_at(input, pos);
                                match ch {
                                    ']' | '[' =>
                                    state.mark_failure(pos, "[^][]"),
                                    _ => Matched(next, ()),
                                }
                            } else { state.mark_failure(pos, "[^][]") };
                        match step_res {
                            Matched(newpos, value) => {
                                repeat_pos = newpos;
                                repeat_value.push(value);
                            }
                            Failed => { break ; }
                        }
                    }
                    Matched(repeat_pos, repeat_value)
                };
            match seq_res {
                Matched(pos, cs) => {
                    {
                        let match_str = &input[start_pos..pos];
                        Matched(pos, { match_str.to_string() })
                    }
                }
                Failed => Failed,
            }
        }
    }
}
fn parse_silent_chars<'input>(input: &'input str, state: &mut ParseState,
                              pos: usize) -> RuleResult<String> {
    {
        let start_pos = pos;
        {
            let seq_res =
                {
                    let mut repeat_pos = pos;
                    let mut repeat_value = vec!();
                    loop  {
                        let pos = repeat_pos;
                        let step_res =
                            if input.len() > pos {
                                let (ch, next) = char_range_at(input, pos);
                                match ch {
                                    ')' | '(' =>
                                    state.mark_failure(pos, "[^)(]"),
                                    _ => Matched(next, ()),
                                }
                            } else { state.mark_failure(pos, "[^)(]") };
                        match step_res {
                            Matched(newpos, value) => {
                                repeat_pos = newpos;
                                repeat_value.push(value);
                            }
                            Failed => { break ; }
                        }
                    }
                    Matched(repeat_pos, repeat_value)
                };
            match seq_res {
                Matched(pos, cs) => {
                    {
                        let match_str = &input[start_pos..pos];
                        Matched(pos, { match_str.to_string() })
                    }
                }
                Failed => Failed,
            }
        }
    }
}
fn parse_modifier<'input>(input: &'input str, state: &mut ParseState,
                          pos: usize) -> RuleResult<KarmaChange> {
    {
        let choice_res =
            {
                let start_pos = pos;
                {
                    let seq_res = slice_eq(input, state, pos, "++");
                    match seq_res {
                        Matched(pos, _) => {
                            {
                                let seq_res =
                                    parse_at_boundary(input, state, pos);
                                match seq_res {
                                    Matched(pos, _) => {
                                        {
                                            let match_str =
                                                &input[start_pos..pos];
                                            Matched(pos,
                                                    {
                                                        KarmaChange{up: 1,
                                                                    down: 0,}
                                                    })
                                        }
                                    }
                                    Failed => Failed,
                                }
                            }
                        }
                        Failed => Failed,
                    }
                }
            };
        match choice_res {
            Matched(pos, value) => Matched(pos, value),
            Failed => {
                let start_pos = pos;
                {
                    let seq_res = slice_eq(input, state, pos, "--");
                    match seq_res {
                        Matched(pos, _) => {
                            {
                                let seq_res =
                                    parse_at_boundary(input, state, pos);
                                match seq_res {
                                    Matched(pos, _) => {
                                        {
                                            let match_str =
                                                &input[start_pos..pos];
                                            Matched(pos,
                                                    {
                                                        KarmaChange{up: 0,
                                                                    down: 1,}
                                                    })
                                        }
                                    }
                                    Failed => Failed,
                                }
                            }
                        }
                        Failed => Failed,
                    }
                }
            }
        }
    }
}
fn parse_at_boundary<'input>(input: &'input str, state: &mut ParseState,
                             pos: usize) -> RuleResult<()> {
    {
        let choice_res =
            {
                let assert_res = parse_whitespace(input, state, pos);
                match assert_res {
                    Matched(..) => Matched(pos, ()),
                    Failed => Failed,
                }
            };
        match choice_res {
            Matched(pos, value) => Matched(pos, value),
            Failed => {
                let choice_res =
                    {
                        let assert_res =
                            if input.len() > pos {
                                let (ch, next) = char_range_at(input, pos);
                                match ch {
                                    ',' | '.' | ';' | ':' | ')' =>
                                    Matched(next, ()),
                                    _ => state.mark_failure(pos, "[,.;:)]"),
                                }
                            } else { state.mark_failure(pos, "[,.;:)]") };
                        match assert_res {
                            Matched(..) => Matched(pos, ()),
                            Failed => Failed,
                        }
                    };
                match choice_res {
                    Matched(pos, value) => Matched(pos, value),
                    Failed => {
                        let assert_res = any_char(input, state, pos);
                        match assert_res {
                            Failed => Matched(pos, ()),
                            Matched(..) => Failed,
                        }
                    }
                }
            }
        }
    }
}
fn parse_whitespace<'input>(input: &'input str, state: &mut ParseState,
                            pos: usize) -> RuleResult<()> {
    {
        let choice_res =
            if input.len() > pos {
                let (ch, next) = char_range_at(input, pos);
                match ch {
                    ' ' | '\t' | '\n' | '\r' | 'v' | 'f' | '\u{85}' =>
                    Matched(next, ()),
                    _ => state.mark_failure(pos, "[ \t\n\rvf\u{85}]"),
                }
            } else { state.mark_failure(pos, "[ \t\n\rvf\u{85}]") };
        match choice_res {
            Matched(pos, value) => Matched(pos, value),
            Failed => parse_adv_whitespace(input, state, pos),
        }
    }
}
fn parse_adv_whitespace<'input>(input: &'input str, state: &mut ParseState,
                                pos: usize) -> RuleResult<()> {
    if input.len() > pos {
        let (ch, next) = char_range_at(input, pos);
        match ch {
            '\u{a0}' | '\u{1680}' | '\u{2000}' | '\u{2001}' | '\u{2002}' |
            '\u{2003}' | '\u{2004}' | '\u{2005}' | '\u{2006}' | '\u{2007}' |
            '\u{2008}' | '\u{2009}' | '\u{200a}' | '\u{2028}' | '\u{2029}' |
            '\u{202f}' | '\u{205f}' | '\u{3000}' => Matched(next, ()),
            _ =>
            state.mark_failure(pos,
                               "[\u{a0}\u{1680}\u{2000}\u{2001}\u{2002}\u{2003}\u{2004}\u{2005}\u{2006}\u{2007}\u{2008}\u{2009}\u{200a}\u{2028}\u{2029}\u{202f}\u{205f}\u{3000}]"),
        }
    } else {
        state.mark_failure(pos,
                           "[\u{a0}\u{1680}\u{2000}\u{2001}\u{2002}\u{2003}\u{2004}\u{2005}\u{2006}\u{2007}\u{2008}\u{2009}\u{200a}\u{2028}\u{2029}\u{202f}\u{205f}\u{3000}]")
    }
}
pub fn line<'input>(input: &'input str) -> ParseResult<Vec<Karma>> {
    let mut state = ParseState::new();
    match parse_line(input, &mut state, 0) {
        Matched(pos, value) => { if pos == input.len() { return Ok(value) } }
        _ => { }
    }
    let (line, col) = pos_to_line(input, state.max_err_pos);
    Err(ParseError{line: line,
                   column: col,
                   offset: state.max_err_pos,
                   expected: state.expected,})
}
