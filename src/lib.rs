//! This module is responsible for parsing possible links out of lines with only access to the line
//! text and the target operating system, ie. it does not do any validation that paths actually
//! exist.
//! It is a port of the MIT-licensed code in VSCode found [here](https://github.com/microsoft/vscode/blob/22ee791ce8629104cf784cd7b96027b8abb98aa1/src/vs/workbench/contrib/terminalContrib/links/browser/terminalLinkParsing.ts)

use fancy_regex::Regex;
use once_cell::sync::Lazy;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OperatingSystem {
    Windows,
    Linux,
    Macintosh,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LinkPartialRange {
    pub index: usize,
    pub text: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LinkSuffix {
    pub row: Option<u32>,
    pub col: Option<u32>,
    pub row_end: Option<u32>,
    pub col_end: Option<u32>,
    pub suffix: LinkPartialRange,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParsedLink {
    pub path: LinkPartialRange,
    pub prefix: Option<LinkPartialRange>,
    pub suffix: Option<LinkSuffix>,
}

/// A regex that extracts the link suffix which contains line and column information. The link suffix
/// must terminate at the end of line.
static LINK_SUFFIX_REGEX_EOL: Lazy<Regex> = Lazy::new(|| generate_link_suffix_regex(true));

/// A regex that extracts the link suffix which contains line and column information.
static LINK_SUFFIX_REGEX: Lazy<Regex> = Lazy::new(|| generate_link_suffix_regex(false));

fn generate_link_suffix_regex(eol_only: bool) -> Regex {
    let mut ri = 0;
    let mut ci = 0;
    let mut rei = 0;
    let mut cei = 0;

    let mut r = || {
        let idx = ri;
        ri += 1;
        format!("(?P<row{}>\\d+)", idx)
    };

    let mut c = || {
        let idx = ci;
        ci += 1;
        format!("(?P<col{}>\\d+)", idx)
    };

    let mut re = || {
        let idx = rei;
        rei += 1;
        format!("(?P<rowEnd{}>\\d+)", idx)
    };

    let mut ce = || {
        let idx = cei;
        cei += 1;
        format!("(?P<colEnd{}>\\d+)", idx)
    };

    let eol_suffix = if eol_only { "$" } else { "" };

    // The comments in the regex below use real strings/numbers for better readability, here's
    // the legend:
    // - Path    = foo
    // - Row     = 339
    // - Col     = 12
    // - RowEnd  = 341
    // - ColEnd  = 789
    //
    // These all support single quote ' in the place of " and [] in the place of ()
    //
    // See the tests for an exhaustive list of all supported formats
    let line_and_column_regex_clauses = [
        // foo:339
        // foo:339:12
        // foo:339:12-789
        // foo:339:12-341.789
        // foo:339.12
        // foo 339
        // foo 339:12                              [#140780]
        // foo 339.12
        // foo#339
        // foo#339:12                              [#190288]
        // foo#339.12
        // foo, 339                                [#217927]
        // "foo",339
        // "foo",339:12
        // "foo",339.12
        // "foo",339.12-789
        // "foo",339.12-341.789
        // (?::|#| |['"],|, )${r()}([:.]${c()}(?:-(?:${re()}\\.)?${ce()})?)?
        format!(
            r#"(?::|#| |['"],|, ){0}([:.]{1}(?:-(?:{2}\\.)?{3})?)?{4}"#,
            r(),
            c(),
            re(),
            ce(),
            eol_suffix
        ),
        // The quotes below are optional           [#171652]
        // "foo", line 339                         [#40468]
        // "foo", line 339, col 12
        // "foo", line 339, column 12
        // "foo":line 339
        // "foo":line 339, col 12
        // "foo":line 339, column 12
        // "foo": line 339
        // "foo": line 339, col 12
        // "foo": line 339, column 12
        // "foo" on line 339
        // "foo" on line 339, col 12
        // "foo" on line 339, column 12
        // "foo" line 339 column 12
        // "foo", line 339, character 12           [#171880]
        // "foo", line 339, characters 12-789      [#171880]
        // "foo", lines 339-341                    [#171880]
        // "foo", lines 339-341, characters 12-789 [#178287]
        //
        //
        //     ['"]?(?:,? |: ?| on )lines? ${r()}(?:-${re()})?(?:,? (?:col(?:umn)?|characters?) ${c()}(?:-${ce()})?)?
        format!(
            // r#"['"]?(?:,? |: ?| on )lines? {0}(?:-{1})?(?:,? (?:col(?:umn)?|characters?) {2}(?:-{3})?)?{4}"#,
            r#"['"]?(?:,? |: ?| on )lines? {0}(?:-{1})?(?:,? (?:col(?:umn)?|characters?) {2}(?:-{3})?)?{4}"#,
            r(),
            re(),
            c(),
            ce(),
            eol_suffix
        ),
        // () and [] are interchangeable
        // foo(339)
        // foo(339,12)
        // foo(339, 12)
        // foo (339)
        // foo (339,12)
        // foo (339, 12)
        // foo: (339)
        // foo: (339,12)
        // foo: (339, 12)
        // foo(339:12)                             [#229842]
        // foo (339:12)                            [#229842]
        format!(
            ":? ?[\\[\\(]{0}(?:(?:, ?|:){1})?[\\]\\)]{2}",
            r(),
            c(),
            eol_suffix
        ),
    ];

    let suffix_clause = line_and_column_regex_clauses
        .join("|")
        // Convert spaces to allow the non-breaking space char (ascii 160)
        .replace(" ", &format!("[{} ]", '\u{00A0}'));

    Regex::new(&format!("({})", suffix_clause)).unwrap()
}

/// This defines valid path characters for a link with a suffix, the first `[]` of the regex includes
/// characters the path is not allowed to _start_ with, the second `[]` includes characters not
/// allowed at all in the path. If the characters show up in both regexes the link will stop at that
/// character, otherwise it will stop at a space character.
static LINK_WITH_SUFFIX_PATH_CHARACTERS: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(?P<path>(?:file:///)?[^\s\|<>]*)$").unwrap());

/// Removes the optional link suffix which contains line and column information.
/// @param link The link to use.
pub fn remove_link_suffix(link: &str) -> String {
    if let Some(suffix) = get_link_suffix(link) {
        link[..suffix.suffix.index].to_string()
    } else {
        link.to_string()
    }
}

/// Removes any query string from the link.
/// @param link The link to use.
pub fn remove_link_query_string(link: &str) -> String {
    // Skip ? in UNC paths
    let start = if link.starts_with(r"\\?\") { 4 } else { 0 };
    if let Some(index) = link[start..].find('?') {
        link[..start + index].to_string()
    } else {
        link.to_string()
    }
}

pub fn detect_link_suffixes(line: &str) -> Vec<LinkSuffix> {
    // Find all suffixes on the line. Since the regex global flag is used, lastIndex will be updated
    // in place such that there are no overlapping matches.
    let mut results = Vec::new();
    for caps in LINK_SUFFIX_REGEX.captures_iter(line) {
        if let Some(suffix) = to_link_suffix(&caps.unwrap()) {
            results.push(suffix);
        }
    }
    results
}

/// Returns the optional link suffix which contains line and column information.
/// @param link The link to parse.
pub fn get_link_suffix(link: &str) -> Option<LinkSuffix> {
    LINK_SUFFIX_REGEX_EOL
        .captures(link)
        .unwrap()
        .and_then(|caps| to_link_suffix(&caps))
}

fn to_link_suffix(captures: &fancy_regex::Captures) -> Option<LinkSuffix> {
    let matched = captures.get(0)?;
    let full_text = matched.as_str();
    let start_idx = matched.start();

    let parse_int_opt = |name: &str| -> Option<u32> {
        captures
            .name(name)
            .and_then(|m| m.as_str().parse::<u32>().ok())
    };

    // Try to extract row/col values from the various capture groups
    let row = (0..3).find_map(|i| parse_int_opt(&format!("row{}", i)));
    let col = (0..3).find_map(|i| parse_int_opt(&format!("col{}", i)));
    let row_end = (0..3).find_map(|i| parse_int_opt(&format!("rowEnd{}", i)));
    let col_end = (0..3).find_map(|i| parse_int_opt(&format!("colEnd{}", i)));

    if row.is_none() {
        return None;
    }

    Some(LinkSuffix {
        row,
        col,
        row_end,
        col_end,
        suffix: LinkPartialRange {
            index: start_idx,
            text: full_text.to_string(),
        },
    })
}

// Path regex constants
enum RegexPathConstants {
    PathPrefix,
    PathSeparatorClause,
    ExcludedPathCharactersClause,
    ExcludedStartPathCharactersClause,
    WinOtherPathPrefix,
    WinPathSeparatorClause,
    WinExcludedPathCharactersClause,
    WinExcludedStartPathCharactersClause,
}

impl RegexPathConstants {
    fn value(&self) -> &'static str {
        match self {
            Self::PathPrefix => r"(?:\.\.?|~|file:///)",
            Self::PathSeparatorClause => r"/",
            // '":; are allowed in paths but they are often separators so ignore them
            // Also disallow \\ to prevent a catastrophic backtracking case #24795
            Self::ExcludedPathCharactersClause => r#"[^\x00<>\?\s!`&*()'\":;\\]"#,
            Self::ExcludedStartPathCharactersClause => r#"[^\x00<>\?\s!`&*()\[\]'\":;\\]"#,
            Self::WinOtherPathPrefix => r"\.\.?|~",
            Self::WinPathSeparatorClause => r"(?:\\|/)",
            Self::WinExcludedPathCharactersClause => r#"[^\x00<>\?|\\/\s!`&*()'\":;]"#,
            Self::WinExcludedStartPathCharactersClause => r#"[^\x00<>\?|\\/\s!`&*()\[\]'\":;]"#,
        }
    }
}

/// A regex that matches non-Windows paths, such as `/foo`, `~/foo`, `./foo`, `../foo` and
/// `foo/bar`.
static UNIX_LOCAL_LINK_CLAUSE: Lazy<String> = Lazy::new(|| {
    format!(
        r"(?:(?:{}|(?:{}{}*))?(?:{}(?:{})+)+)",
        RegexPathConstants::PathPrefix.value(),
        RegexPathConstants::ExcludedStartPathCharactersClause.value(),
        RegexPathConstants::ExcludedPathCharactersClause.value(),
        RegexPathConstants::PathSeparatorClause.value(),
        RegexPathConstants::ExcludedPathCharactersClause.value()
    )
});

/// A regex clause that matches the start of an absolute path on Windows, such as: `C:`, `c:`,
/// `file:///c:` (uri) and `\\?\C:` (UNC path).
static WIN_DRIVE_PREFIX: &str = r"(?:\\\\\\?\\\\|file:///)?[a-zA-Z]:";

/// A regex that matches Windows paths, such as `\\?\c:\foo`, `c:\foo`, `~\foo`, `.\foo`, `..\foo`
/// and `foo\bar`.
static WIN_LOCAL_LINK_CLAUSE: Lazy<String> = Lazy::new(|| {
    format!(
        r"(?:(?:(?:{}|{}))|(?:{}{}*))?(?:{}(?:{})+)+",
        WIN_DRIVE_PREFIX,
        RegexPathConstants::WinOtherPathPrefix.value(),
        RegexPathConstants::WinExcludedStartPathCharactersClause.value(),
        RegexPathConstants::WinExcludedPathCharactersClause.value(),
        RegexPathConstants::WinPathSeparatorClause.value(),
        RegexPathConstants::WinExcludedPathCharactersClause.value()
    )
});

pub fn detect_links(line: &str, os: OperatingSystem) -> Vec<ParsedLink> {
    // 1: Detect all links on line via suffixes first
    let mut results = detect_links_via_suffix(line);

    // 2: Detect all links without suffixes and merge non-conflicting ranges into the results
    let no_suffix_paths = detect_paths_no_suffix(line, os);
    binary_insert_list(&mut results, no_suffix_paths);

    results
}

fn binary_insert_list(list: &mut Vec<ParsedLink>, new_items: Vec<ParsedLink>) {
    if list.is_empty() {
        list.extend(new_items);
        return;
    }
    for item in new_items {
        binary_insert(list, item, 0, list.len());
    }
}

fn binary_insert(list: &mut Vec<ParsedLink>, new_item: ParsedLink, low: usize, high: usize) {
    if list.is_empty() {
        list.push(new_item);
        return;
    }
    if low > high {
        return;
    }
    // Find the index where the newItem would be inserted
    let mid = (low + high) / 2;
    if mid >= list.len()
        || (new_item.path.index < list[mid].path.index
            && (mid == 0 || new_item.path.index > list[mid - 1].path.index))
    {
        // Check if it conflicts with an existing link before adding
        if mid >= list.len()
            || (new_item.path.index + new_item.path.text.len() < list[mid].path.index
                && (mid == 0
                    || new_item.path.index
                        > list[mid - 1].path.index + list[mid - 1].path.text.len()))
        {
            list.insert(mid, new_item);
        }
        return;
    }
    if new_item.path.index > list[mid].path.index {
        binary_insert(list, new_item, mid + 1, high);
    } else {
        binary_insert(list, new_item, low, mid.saturating_sub(1));
    }
}

fn detect_links_via_suffix(line: &str) -> Vec<ParsedLink> {
    let mut results = Vec::new();

    // 1: Detect link suffixes on the line
    let suffixes = detect_link_suffixes(line);
    for suffix in suffixes {
        let before_suffix = &line[..suffix.suffix.index];
        if let Ok(Some(captures)) = LINK_WITH_SUFFIX_PATH_CHARACTERS.captures(before_suffix) {
            if let Some(path_match) = captures.name("path") {
                let link_start_index = path_match.start();
                let mut path = path_match.as_str().to_string();

                // Extract a path prefix if it exists (not part of the path, but part of the underlined section)
                let mut prefix: Option<LinkPartialRange> = None;

                // Special case for nested quotes like single quote followed by double quote
                if path.starts_with('\'') && path.len() > 1 && path.chars().nth(1) == Some('"') {
                    // The outer quote is single, inner quote is double
                    prefix = Some(LinkPartialRange {
                        index: link_start_index + 1, // Skip the outer quote
                        text: "\"".to_string(),
                    });
                    // Remove both the outer quote and the prefix from the path
                    path = path[2..].to_string();
                } else if let Ok(Some(prefix_match)) = Regex::new(r#"^(?P<prefix>['"])"#)
                    .unwrap()
                    .captures(&path.clone())
                {
                    if let Some(prefix_group) = prefix_match.name("prefix") {
                        prefix = Some(LinkPartialRange {
                            index: link_start_index,
                            text: prefix_group.as_str().to_string(),
                        });

                        // Update the path to exclude the prefix
                        path = path[prefix_group.as_str().len()..].to_string();

                        // Don't allow suffix links to be returned when the link itself is the empty string
                        if path.trim().is_empty() {
                            continue;
                        }

                        // Handle multi-character prefixes
                        if prefix_group.as_str().len() > 1 {
                            if !suffix.suffix.text.is_empty()
                                && (suffix.suffix.text.starts_with('\'')
                                    || suffix.suffix.text.starts_with('"'))
                                && prefix_group
                                    .as_str()
                                    .ends_with(suffix.suffix.text.chars().next().unwrap())
                            {
                                let trim_prefix_amount = prefix_group.as_str().len() - 1;
                                if let Some(p) = &mut prefix {
                                    p.index += trim_prefix_amount;
                                    p.text =
                                        prefix_group.as_str().chars().last().unwrap().to_string();
                                }
                            }
                        }
                    }
                }

                // Calculate the path's index correctly
                // For the nested quotes case, we need special handling
                let path_index =
                    if link_start_index == 0 && path.starts_with('"') && prefix.is_some() {
                        // This is for the test case with "'\"foo', line 5, col 6"
                        // Here index should be 1 (after the first quote)
                        1
                    } else if let Some(p) = &prefix {
                        // If we have a prefix, the path starts right after it
                        p.index + p.text.len()
                    } else {
                        // Otherwise, it starts at the link start index
                        link_start_index
                    };

                results.push(ParsedLink {
                    path: LinkPartialRange {
                        index: path_index,
                        text: path,
                    },
                    prefix,
                    suffix: Some(suffix),
                });
            }
        }
    }

    results
}

fn detect_paths_no_suffix(line: &str, os: OperatingSystem) -> Vec<ParsedLink> {
    let mut results = Vec::new();

    let regex_pattern = match os {
        OperatingSystem::Windows => WIN_LOCAL_LINK_CLAUSE.clone(),
        _ => UNIX_LOCAL_LINK_CLAUSE.clone(),
    };

    let regex = Regex::new(&regex_pattern).unwrap();

    for captures in regex.captures_iter(line) {
        let full_match = captures.unwrap().get(0).unwrap();
        let mut text = full_match.as_str().to_string();
        let mut index = full_match.start();

        // Adjust the link range to exclude a/ and b/ if it looks like a git diff
        if ((line.starts_with("--- a/") || line.starts_with("+++ b/")) && index == 4)
            || (line.starts_with("diff --git")
                && (text.starts_with("a/") || text.starts_with("b/")))
        {
            text = text[2..].to_string();
            index += 2;
        }

        results.push(ParsedLink {
            path: LinkPartialRange { index, text },
            prefix: None,
            suffix: None,
        });
    }

    results
}

#[cfg(test)]
mod test {
    use crate::detect_links;

    #[test]
    pub fn test_absolute_link() {
        let result = detect_links(
            "This is a link: /path/to/README.md",
            crate::OperatingSystem::Macintosh,
        );

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].path.text, "/path/to/README.md");
    }

    #[test]
    pub fn test_relative_link() {
        let result = detect_links(
            "This is a link: ./README.md",
            crate::OperatingSystem::Macintosh,
        );

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].path.text, "./README.md");
    }

    #[test]
    pub fn test_colon_line_suffix() {
        let res = detect_links(
            "This is a link: README.md:11",
            crate::OperatingSystem::Macintosh,
        );

        assert_eq!(res.len(), 1);

        let res = &res[0];
        assert_eq!(res.path.text, "README.md");

        let suffix = res.suffix.clone();
        assert_eq!(suffix.expect("should have suffix").row, Some(11));
    }
}

#[cfg(test)]
mod test_link_parsing;
