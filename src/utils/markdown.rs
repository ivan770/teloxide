//! Utils for working with the [Markdown V2 message style][spec].
//!
//! [spec]: https://core.telegram.org/bots/api#markdownv2-style
use crate::types::User;
use std::string::String;

/// Applies the bold font style to the string.
///
/// Passed string will not be automatically escaped because it can contain
/// nested markup.
pub fn bold(s: &str) -> String {
    format!("*{}*", s)
}

/// Applies the italic font style to the string.
///
/// Can be safely used with `utils::markdown::underline()`.
/// Passed string will not be automatically escaped because it can contain
/// nested markup.
pub fn italic(s: &str) -> String {
    if s.starts_with("__") && s.ends_with("__") {
        format!(r"_{}\r__", &s[..s.len() - 1])
    } else {
        format!("_{}_", s)
    }
}

/// Applies the underline font style to the string.
///
/// Can be safely used with `utils::markdown::italic()`.
/// Passed string will not be automatically escaped because it can contain
/// nested markup.
pub fn underline(s: &str) -> String {
    // In case of ambiguity between italic and underline entities
    // ‘__’ is always greadily treated from left to right as beginning or end of
    // underline entity, so instead of ___italic underline___ we should use
    // ___italic underline_\r__, where \r is a character with code 13, which
    // will be ignored.
    if s.starts_with('_') && s.ends_with('_') {
        format!(r"__{}\r__", s)
    } else {
        format!("__{}__", s)
    }
}

/// Applies the strikethrough font style to the string.
///
/// Passed string will not be automatically escaped because it can contain
/// nested markup.
pub fn strike(s: &str) -> String {
    format!("~{}~", s)
}

/// Builds an inline link with an anchor.
///
/// Escapes `)` and ``` characters inside the link url.
pub fn link(url: &str, text: &str) -> String {
    format!("[{}]({})", text, escape_link_url(url))
}

/// Builds an inline user mention link with an anchor.
pub fn user_mention(user_id: i32, text: &str) -> String {
    link(format!("tg://user?id={}", user_id).as_str(), text)
}

/// Formats the code block.
///
/// Escapes ``` and `\` characters inside the block.
pub fn code_block(code: &str) -> String {
    format!("```\n{}\n```", escape_code(code))
}

/// Formats the code block with a specific language syntax.
///
/// Escapes ``` and `\` characters inside the block.
pub fn code_block_with_lang(code: &str, lang: &str) -> String {
    format!("```{}\n{}\n```", escape(lang), escape_code(code))
}

/// Formats the string as an inline code.
///
/// Escapes ``` and `\` characters inside the block.
pub fn code_inline(s: &str) -> String {
    format!("`{}`", escape_code(s))
}

/// Escapes the string to be shown "as is" within the Telegram [Markdown
/// v2][spec] message style.
///
/// [spec]: https://core.telegram.org/bots/api#html-style
pub fn escape(s: &str) -> String {
    s.replace("_", r"\_")
        .replace("*", r"\*")
        .replace("[", r"\[")
        .replace("]", r"\]")
        .replace("(", r"\(")
        .replace(")", r"\)")
        .replace("~", r"\~")
        .replace("`", r"\`")
        .replace(">", r"\>")
        .replace("#", r"\#")
        .replace("+", r"\+")
        .replace("-", r"\-")
        .replace("=", r"\=")
        .replace("|", r"\|")
        .replace("{", r"\{")
        .replace("}", r"\}")
        .replace(".", r"\.")
        .replace("!", r"\!")
}

/// Escapes all markdown special characters specific for the inline link URL
/// (``` and `)`).
pub fn escape_link_url(s: &str) -> String {
    s.replace("`", r"\`").replace(")", r"\)")
}

/// Escapes all markdown special characters specific for the code block (``` and
/// `\`).
pub fn escape_code(s: &str) -> String {
    s.replace(r"\", r"\\").replace("`", r"\`")
}

pub fn user_mention_or_link(user: &User) -> String {
    match user.mention() {
        Some(mention) => mention,
        None => link(user.url().as_str(), &user.full_name()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bold() {
        assert_eq!(bold(" foobar "), "* foobar *");
        assert_eq!(bold(" _foobar_ "), "* _foobar_ *");
        assert_eq!(bold("~(`foobar`)~"), "*~(`foobar`)~*");
    }

    #[test]
    fn test_italic() {
        assert_eq!(italic(" foobar "), "_ foobar _");
        assert_eq!(italic("*foobar*"), "_*foobar*_");
        assert_eq!(italic("~(foobar)~"), "_~(foobar)~_");
    }

    #[test]
    fn test_underline() {
        assert_eq!(underline(" foobar "), "__ foobar __");
        assert_eq!(underline("*foobar*"), "__*foobar*__");
        assert_eq!(underline("~(foobar)~"), "__~(foobar)~__");
    }

    #[test]
    fn test_strike() {
        assert_eq!(strike(" foobar "), "~ foobar ~");
        assert_eq!(strike("*foobar*"), "~*foobar*~");
        assert_eq!(strike("*(foobar)*"), "~*(foobar)*~");
    }

    #[test]
    fn test_italic_with_underline() {
        assert_eq!(underline(italic("foobar").as_str()), r"___foobar_\r__");
        assert_eq!(italic(underline("foobar").as_str()), r"___foobar_\r__");
    }

    #[test]
    fn test_link() {
        assert_eq!(
            link("https://www.google.com/(`foobar`)", "google"),
            r"[google](https://www.google.com/(\`foobar\`\))",
        );
    }

    #[test]
    fn test_user_mention() {
        assert_eq!(user_mention(123_456_789, "pwner666"), "[pwner666](tg://user?id=123456789)");
    }

    #[test]
    fn test_code_block() {
        assert_eq!(
            code_block("pre-'formatted'\nfixed-width \\code `block`"),
            "```\npre-'formatted'\nfixed-width \\\\code \\`block\\`\n```"
        );
    }

    #[test]
    fn test_code_block_with_lang() {
        assert_eq!(
            code_block_with_lang("pre-'formatted'\nfixed-width \\code `block`", "[python]"),
            "```\\[python\\]\npre-'formatted'\nfixed-width \\\\code \\`block\\`\n```"
        );
    }

    #[test]
    fn test_code_inline() {
        assert_eq!(code_inline(" let x = (1, 2, 3); "), "` let x = (1, 2, 3); `");
        assert_eq!(code_inline("<html>foo</html>"), "`<html>foo</html>`");
        assert_eq!(code_inline(r" `(code inside code \ )` "), r"` \`(code inside code \\ )\` `");
    }

    #[test]
    fn test_escape() {
        assert_eq!(escape("* foobar *"), r"\* foobar \*");
        assert_eq!(
            escape(r"_ * [ ] ( ) ~ \ ` > # + - = | { } . !"),
            r"\_ \* \[ \] \( \) \~ \ \` \> \# \+ \- \= \| \{ \} \. \!",
        );
    }

    #[test]
    fn test_escape_link_url() {
        assert_eq!(
            escape_link_url(r"https://en.wikipedia.org/wiki/Development+(Software)"),
            r"https://en.wikipedia.org/wiki/Development+(Software\)"
        );
        assert_eq!(
            escape_link_url(r"https://en.wikipedia.org/wiki/`"),
            r"https://en.wikipedia.org/wiki/\`"
        );
        assert_eq!(escape_link_url(r"_*[]()~`#+-=|{}.!\"), r"_*[](\)~\`#+-=|{}.!\");
    }

    #[test]
    fn test_escape_code() {
        assert_eq!(escape_code(r"` \code inside the code\ `"), r"\` \\code inside the code\\ \`");
        assert_eq!(escape_code(r"_*[]()~`#+-=|{}.!\"), r"_*[]()~\`#+-=|{}.!\\");
    }

    #[test]
    fn user_mention_link() {
        let user_with_username = User {
            id: 0,
            is_bot: false,
            first_name: "".to_string(),
            last_name: None,
            username: Some("abcd".to_string()),
            language_code: None,
        };
        assert_eq!(user_mention_or_link(&user_with_username), "@abcd");
        let user_without_username = User {
            id: 123_456_789,
            is_bot: false,
            first_name: "Name".to_string(),
            last_name: None,
            username: None,
            language_code: None,
        };
        assert_eq!(
            user_mention_or_link(&user_without_username),
            r#"[Name](tg://user/?id=123456789)"#
        )
    }
}
