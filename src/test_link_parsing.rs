#[cfg(test)]
mod link_parsing_tests {
    use crate::{
        LinkPartialRange, LinkSuffix, OperatingSystem, detect_link_suffixes, detect_links,
        get_link_suffix, remove_link_query_string, remove_link_suffix,
    };
    use pretty_assertions::assert_eq;
    use std::fmt;

    const TEST_ROW: u32 = 339;
    const TEST_COL: u32 = 12;
    const TEST_ROW_END: u32 = 341;
    const TEST_COL_END: u32 = 789;

    #[allow(dead_code)]
    struct TestLink {
        link: &'static str,
        prefix: Option<&'static str>,
        suffix: Option<&'static str>,
        has_row: bool,
        has_col: bool,
        has_row_end: bool,
        has_col_end: bool,
    }

    impl fmt::Display for TestLink {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.link)
        }
    }

    const OS_TEST_PATHS: [&str; 3] = [
        "/test/path/linux",        // Linux
        "/test/path/macintosh",    // Macintosh
        "C:\\test\\path\\windows", // Windows
    ];

    const TEST_LINKS: &[TestLink] = &[
        // Simple
        TestLink {
            link: "foo",
            prefix: None,
            suffix: None,
            has_row: false,
            has_col: false,
            has_row_end: false,
            has_col_end: false,
        },
        TestLink {
            link: "foo:339",
            prefix: None,
            suffix: Some(":339"),
            has_row: true,
            has_col: false,
            has_row_end: false,
            has_col_end: false,
        },
        TestLink {
            link: "foo:339:12",
            prefix: None,
            suffix: Some(":339:12"),
            has_row: true,
            has_col: true,
            has_row_end: false,
            has_col_end: false,
        },
        TestLink {
            link: "foo:339:12-789",
            prefix: None,
            suffix: Some(":339:12-789"),
            has_row: true,
            has_col: true,
            has_row_end: false,
            has_col_end: true,
        },
        TestLink {
            link: "foo:339.12",
            prefix: None,
            suffix: Some(":339.12"),
            has_row: true,
            has_col: true,
            has_row_end: false,
            has_col_end: false,
        },
        TestLink {
            link: "foo:339.12-789",
            prefix: None,
            suffix: Some(":339.12-789"),
            has_row: true,
            has_col: true,
            has_row_end: false,
            has_col_end: true,
        },
        TestLink {
            link: "foo:339.12-341.789",
            prefix: None,
            suffix: Some(":339.12-341.789"),
            has_row: true,
            has_col: true,
            has_row_end: true,
            has_col_end: true,
        },
        TestLink {
            link: "foo#339",
            prefix: None,
            suffix: Some("#339"),
            has_row: true,
            has_col: false,
            has_row_end: false,
            has_col_end: false,
        },
        TestLink {
            link: "foo#339:12",
            prefix: None,
            suffix: Some("#339:12"),
            has_row: true,
            has_col: true,
            has_row_end: false,
            has_col_end: false,
        },
        TestLink {
            link: "foo#339:12-789",
            prefix: None,
            suffix: Some("#339:12-789"),
            has_row: true,
            has_col: true,
            has_row_end: false,
            has_col_end: true,
        },
        TestLink {
            link: "foo#339.12",
            prefix: None,
            suffix: Some("#339.12"),
            has_row: true,
            has_col: true,
            has_row_end: false,
            has_col_end: false,
        },
        TestLink {
            link: "foo#339.12-789",
            prefix: None,
            suffix: Some("#339.12-789"),
            has_row: true,
            has_col: true,
            has_row_end: false,
            has_col_end: true,
        },
        TestLink {
            link: "foo#339.12-341.789",
            prefix: None,
            suffix: Some("#339.12-341.789"),
            has_row: true,
            has_col: true,
            has_row_end: true,
            has_col_end: true,
        },
        TestLink {
            link: "foo 339",
            prefix: None,
            suffix: Some(" 339"),
            has_row: true,
            has_col: false,
            has_row_end: false,
            has_col_end: false,
        },
        TestLink {
            link: "foo 339:12",
            prefix: None,
            suffix: Some(" 339:12"),
            has_row: true,
            has_col: true,
            has_row_end: false,
            has_col_end: false,
        },
        TestLink {
            link: "foo 339:12-789",
            prefix: None,
            suffix: Some(" 339:12-789"),
            has_row: true,
            has_col: true,
            has_row_end: false,
            has_col_end: true,
        },
        TestLink {
            link: "foo 339.12",
            prefix: None,
            suffix: Some(" 339.12"),
            has_row: true,
            has_col: true,
            has_row_end: false,
            has_col_end: false,
        },
        TestLink {
            link: "foo 339.12-789",
            prefix: None,
            suffix: Some(" 339.12-789"),
            has_row: true,
            has_col: true,
            has_row_end: false,
            has_col_end: true,
        },
        TestLink {
            link: "foo 339.12-341.789",
            prefix: None,
            suffix: Some(" 339.12-341.789"),
            has_row: true,
            has_col: true,
            has_row_end: true,
            has_col_end: true,
        },
        TestLink {
            link: "foo, 339",
            prefix: None,
            suffix: Some(", 339"),
            has_row: true,
            has_col: false,
            has_row_end: false,
            has_col_end: false,
        },
        // Double quotes
        TestLink {
            link: "\"foo\",339",
            prefix: Some("\""),
            suffix: Some("\",339"),
            has_row: true,
            has_col: false,
            has_row_end: false,
            has_col_end: false,
        },
        TestLink {
            link: "\"foo\",339:12",
            prefix: Some("\""),
            suffix: Some("\",339:12"),
            has_row: true,
            has_col: true,
            has_row_end: false,
            has_col_end: false,
        },
        TestLink {
            link: "\"foo\",339.12",
            prefix: Some("\""),
            suffix: Some("\",339.12"),
            has_row: true,
            has_col: true,
            has_row_end: false,
            has_col_end: false,
        },
        TestLink {
            link: "\"foo\", line 339",
            prefix: Some("\""),
            suffix: Some("\", line 339"),
            has_row: true,
            has_col: false,
            has_row_end: false,
            has_col_end: false,
        },
        TestLink {
            link: "\"foo\", line 339, col 12",
            prefix: Some("\""),
            suffix: Some("\", line 339, col 12"),
            has_row: true,
            has_col: true,
            has_row_end: false,
            has_col_end: false,
        },
        TestLink {
            link: "\"foo\", line 339, column 12",
            prefix: Some("\""),
            suffix: Some("\", line 339, column 12"),
            has_row: true,
            has_col: true,
            has_row_end: false,
            has_col_end: false,
        },
        TestLink {
            link: "\"foo\":line 339",
            prefix: Some("\""),
            suffix: Some("\":line 339"),
            has_row: true,
            has_col: false,
            has_row_end: false,
            has_col_end: false,
        },
        TestLink {
            link: "\"foo\":line 339, col 12",
            prefix: Some("\""),
            suffix: Some("\":line 339, col 12"),
            has_row: true,
            has_col: true,
            has_row_end: false,
            has_col_end: false,
        },
        TestLink {
            link: "\"foo\":line 339, column 12",
            prefix: Some("\""),
            suffix: Some("\":line 339, column 12"),
            has_row: true,
            has_col: true,
            has_row_end: false,
            has_col_end: false,
        },
        TestLink {
            link: "\"foo\": line 339",
            prefix: Some("\""),
            suffix: Some("\": line 339"),
            has_row: true,
            has_col: false,
            has_row_end: false,
            has_col_end: false,
        },
        TestLink {
            link: "\"foo\": line 339, col 12",
            prefix: Some("\""),
            suffix: Some("\": line 339, col 12"),
            has_row: true,
            has_col: true,
            has_row_end: false,
            has_col_end: false,
        },
        TestLink {
            link: "\"foo\": line 339, column 12",
            prefix: Some("\""),
            suffix: Some("\": line 339, column 12"),
            has_row: true,
            has_col: true,
            has_row_end: false,
            has_col_end: false,
        },
        TestLink {
            link: "\"foo\" on line 339",
            prefix: Some("\""),
            suffix: Some("\" on line 339"),
            has_row: true,
            has_col: false,
            has_row_end: false,
            has_col_end: false,
        },
        TestLink {
            link: "\"foo\" on line 339, col 12",
            prefix: Some("\""),
            suffix: Some("\" on line 339, col 12"),
            has_row: true,
            has_col: true,
            has_row_end: false,
            has_col_end: false,
        },
        TestLink {
            link: "\"foo\" on line 339, column 12",
            prefix: Some("\""),
            suffix: Some("\" on line 339, column 12"),
            has_row: true,
            has_col: true,
            has_row_end: false,
            has_col_end: false,
        },
        TestLink {
            link: "\"foo\" line 339",
            prefix: Some("\""),
            suffix: Some("\" line 339"),
            has_row: true,
            has_col: false,
            has_row_end: false,
            has_col_end: false,
        },
        TestLink {
            link: "\"foo\" line 339 column 12",
            prefix: Some("\""),
            suffix: Some("\" line 339 column 12"),
            has_row: true,
            has_col: true,
            has_row_end: false,
            has_col_end: false,
        },
        // Single quotes
        TestLink {
            link: "'foo',339",
            prefix: Some("'"),
            suffix: Some("',339"),
            has_row: true,
            has_col: false,
            has_row_end: false,
            has_col_end: false,
        },
        TestLink {
            link: "'foo',339:12",
            prefix: Some("'"),
            suffix: Some("',339:12"),
            has_row: true,
            has_col: true,
            has_row_end: false,
            has_col_end: false,
        },
        TestLink {
            link: "'foo',339.12",
            prefix: Some("'"),
            suffix: Some("',339.12"),
            has_row: true,
            has_col: true,
            has_row_end: false,
            has_col_end: false,
        },
        TestLink {
            link: "'foo', line 339",
            prefix: Some("'"),
            suffix: Some("', line 339"),
            has_row: true,
            has_col: false,
            has_row_end: false,
            has_col_end: false,
        },
        TestLink {
            link: "'foo', line 339, col 12",
            prefix: Some("'"),
            suffix: Some("', line 339, col 12"),
            has_row: true,
            has_col: true,
            has_row_end: false,
            has_col_end: false,
        },
        TestLink {
            link: "'foo', line 339, column 12",
            prefix: Some("'"),
            suffix: Some("', line 339, column 12"),
            has_row: true,
            has_col: true,
            has_row_end: false,
            has_col_end: false,
        },
        TestLink {
            link: "'foo':line 339",
            prefix: Some("'"),
            suffix: Some("':line 339"),
            has_row: true,
            has_col: false,
            has_row_end: false,
            has_col_end: false,
        },
        TestLink {
            link: "'foo':line 339, col 12",
            prefix: Some("'"),
            suffix: Some("':line 339, col 12"),
            has_row: true,
            has_col: true,
            has_row_end: false,
            has_col_end: false,
        },
        TestLink {
            link: "'foo':line 339, column 12",
            prefix: Some("'"),
            suffix: Some("':line 339, column 12"),
            has_row: true,
            has_col: true,
            has_row_end: false,
            has_col_end: false,
        },
        TestLink {
            link: "'foo': line 339",
            prefix: Some("'"),
            suffix: Some("': line 339"),
            has_row: true,
            has_col: false,
            has_row_end: false,
            has_col_end: false,
        },
        TestLink {
            link: "'foo': line 339, col 12",
            prefix: Some("'"),
            suffix: Some("': line 339, col 12"),
            has_row: true,
            has_col: true,
            has_row_end: false,
            has_col_end: false,
        },
        TestLink {
            link: "'foo': line 339, column 12",
            prefix: Some("'"),
            suffix: Some("': line 339, column 12"),
            has_row: true,
            has_col: true,
            has_row_end: false,
            has_col_end: false,
        },
        TestLink {
            link: "'foo' on line 339",
            prefix: Some("'"),
            suffix: Some("' on line 339"),
            has_row: true,
            has_col: false,
            has_row_end: false,
            has_col_end: false,
        },
        TestLink {
            link: "'foo' on line 339, col 12",
            prefix: Some("'"),
            suffix: Some("' on line 339, col 12"),
            has_row: true,
            has_col: true,
            has_row_end: false,
            has_col_end: false,
        },
        TestLink {
            link: "'foo' on line 339, column 12",
            prefix: Some("'"),
            suffix: Some("' on line 339, column 12"),
            has_row: true,
            has_col: true,
            has_row_end: false,
            has_col_end: false,
        },
        TestLink {
            link: "'foo' line 339",
            prefix: Some("'"),
            suffix: Some("' line 339"),
            has_row: true,
            has_col: false,
            has_row_end: false,
            has_col_end: false,
        },
        TestLink {
            link: "'foo' line 339 column 12",
            prefix: Some("'"),
            suffix: Some("' line 339 column 12"),
            has_row: true,
            has_col: true,
            has_row_end: false,
            has_col_end: false,
        },
        // No quotes
        TestLink {
            link: "foo, line 339",
            prefix: None,
            suffix: Some(", line 339"),
            has_row: true,
            has_col: false,
            has_row_end: false,
            has_col_end: false,
        },
        TestLink {
            link: "foo, line 339, col 12",
            prefix: None,
            suffix: Some(", line 339, col 12"),
            has_row: true,
            has_col: true,
            has_row_end: false,
            has_col_end: false,
        },
        TestLink {
            link: "foo, line 339, column 12",
            prefix: None,
            suffix: Some(", line 339, column 12"),
            has_row: true,
            has_col: true,
            has_row_end: false,
            has_col_end: false,
        },
        TestLink {
            link: "foo:line 339",
            prefix: None,
            suffix: Some(":line 339"),
            has_row: true,
            has_col: false,
            has_row_end: false,
            has_col_end: false,
        },
        TestLink {
            link: "foo:line 339, col 12",
            prefix: None,
            suffix: Some(":line 339, col 12"),
            has_row: true,
            has_col: true,
            has_row_end: false,
            has_col_end: false,
        },
        TestLink {
            link: "foo:line 339, column 12",
            prefix: None,
            suffix: Some(":line 339, column 12"),
            has_row: true,
            has_col: true,
            has_row_end: false,
            has_col_end: false,
        },
        TestLink {
            link: "foo: line 339",
            prefix: None,
            suffix: Some(": line 339"),
            has_row: true,
            has_col: false,
            has_row_end: false,
            has_col_end: false,
        },
        TestLink {
            link: "foo: line 339, col 12",
            prefix: None,
            suffix: Some(": line 339, col 12"),
            has_row: true,
            has_col: true,
            has_row_end: false,
            has_col_end: false,
        },
        TestLink {
            link: "foo: line 339, column 12",
            prefix: None,
            suffix: Some(": line 339, column 12"),
            has_row: true,
            has_col: true,
            has_row_end: false,
            has_col_end: false,
        },
        TestLink {
            link: "foo on line 339",
            prefix: None,
            suffix: Some(" on line 339"),
            has_row: true,
            has_col: false,
            has_row_end: false,
            has_col_end: false,
        },
        TestLink {
            link: "foo on line 339, col 12",
            prefix: None,
            suffix: Some(" on line 339, col 12"),
            has_row: true,
            has_col: true,
            has_row_end: false,
            has_col_end: false,
        },
        TestLink {
            link: "foo on line 339, column 12",
            prefix: None,
            suffix: Some(" on line 339, column 12"),
            has_row: true,
            has_col: true,
            has_row_end: false,
            has_col_end: false,
        },
        TestLink {
            link: "foo line 339",
            prefix: None,
            suffix: Some(" line 339"),
            has_row: true,
            has_col: false,
            has_row_end: false,
            has_col_end: false,
        },
        TestLink {
            link: "foo line 339 column 12",
            prefix: None,
            suffix: Some(" line 339 column 12"),
            has_row: true,
            has_col: true,
            has_row_end: false,
            has_col_end: false,
        },
        // Parentheses
        TestLink {
            link: "foo(339)",
            prefix: None,
            suffix: Some("(339)"),
            has_row: true,
            has_col: false,
            has_row_end: false,
            has_col_end: false,
        },
        TestLink {
            link: "foo(339,12)",
            prefix: None,
            suffix: Some("(339,12)"),
            has_row: true,
            has_col: true,
            has_row_end: false,
            has_col_end: false,
        },
        TestLink {
            link: "foo(339, 12)",
            prefix: None,
            suffix: Some("(339, 12)"),
            has_row: true,
            has_col: true,
            has_row_end: false,
            has_col_end: false,
        },
        TestLink {
            link: "foo (339)",
            prefix: None,
            suffix: Some(" (339)"),
            has_row: true,
            has_col: false,
            has_row_end: false,
            has_col_end: false,
        },
        TestLink {
            link: "foo (339,12)",
            prefix: None,
            suffix: Some(" (339,12)"),
            has_row: true,
            has_col: true,
            has_row_end: false,
            has_col_end: false,
        },
        TestLink {
            link: "foo (339, 12)",
            prefix: None,
            suffix: Some(" (339, 12)"),
            has_row: true,
            has_col: true,
            has_row_end: false,
            has_col_end: false,
        },
        TestLink {
            link: "foo: (339)",
            prefix: None,
            suffix: Some(": (339)"),
            has_row: true,
            has_col: false,
            has_row_end: false,
            has_col_end: false,
        },
        TestLink {
            link: "foo: (339,12)",
            prefix: None,
            suffix: Some(": (339,12)"),
            has_row: true,
            has_col: true,
            has_row_end: false,
            has_col_end: false,
        },
        TestLink {
            link: "foo: (339, 12)",
            prefix: None,
            suffix: Some(": (339, 12)"),
            has_row: true,
            has_col: true,
            has_row_end: false,
            has_col_end: false,
        },
        TestLink {
            link: "foo(339:12)",
            prefix: None,
            suffix: Some("(339:12)"),
            has_row: true,
            has_col: true,
            has_row_end: false,
            has_col_end: false,
        },
        TestLink {
            link: "foo (339:12)",
            prefix: None,
            suffix: Some(" (339:12)"),
            has_row: true,
            has_col: true,
            has_row_end: false,
            has_col_end: false,
        },
        // Square brackets
        TestLink {
            link: "foo[339]",
            prefix: None,
            suffix: Some("[339]"),
            has_row: true,
            has_col: false,
            has_row_end: false,
            has_col_end: false,
        },
        TestLink {
            link: "foo[339,12]",
            prefix: None,
            suffix: Some("[339,12]"),
            has_row: true,
            has_col: true,
            has_row_end: false,
            has_col_end: false,
        },
        TestLink {
            link: "foo[339, 12]",
            prefix: None,
            suffix: Some("[339, 12]"),
            has_row: true,
            has_col: true,
            has_row_end: false,
            has_col_end: false,
        },
        TestLink {
            link: "foo [339]",
            prefix: None,
            suffix: Some(" [339]"),
            has_row: true,
            has_col: false,
            has_row_end: false,
            has_col_end: false,
        },
        TestLink {
            link: "foo [339,12]",
            prefix: None,
            suffix: Some(" [339,12]"),
            has_row: true,
            has_col: true,
            has_row_end: false,
            has_col_end: false,
        },
        TestLink {
            link: "foo [339, 12]",
            prefix: None,
            suffix: Some(" [339, 12]"),
            has_row: true,
            has_col: true,
            has_row_end: false,
            has_col_end: false,
        },
        TestLink {
            link: "foo: [339]",
            prefix: None,
            suffix: Some(": [339]"),
            has_row: true,
            has_col: false,
            has_row_end: false,
            has_col_end: false,
        },
        TestLink {
            link: "foo: [339,12]",
            prefix: None,
            suffix: Some(": [339,12]"),
            has_row: true,
            has_col: true,
            has_row_end: false,
            has_col_end: false,
        },
        TestLink {
            link: "foo: [339, 12]",
            prefix: None,
            suffix: Some(": [339, 12]"),
            has_row: true,
            has_col: true,
            has_row_end: false,
            has_col_end: false,
        },
        TestLink {
            link: "foo[339:12]",
            prefix: None,
            suffix: Some("[339:12]"),
            has_row: true,
            has_col: true,
            has_row_end: false,
            has_col_end: false,
        },
        TestLink {
            link: "foo [339:12]",
            prefix: None,
            suffix: Some(" [339:12]"),
            has_row: true,
            has_col: true,
            has_row_end: false,
            has_col_end: false,
        },
        // OCaml-style
        TestLink {
            link: "\"foo\", line 339, character 12",
            prefix: Some("\""),
            suffix: Some("\", line 339, character 12"),
            has_row: true,
            has_col: true,
            has_row_end: false,
            has_col_end: false,
        },
        TestLink {
            link: "\"foo\", line 339, characters 12-789",
            prefix: Some("\""),
            suffix: Some("\", line 339, characters 12-789"),
            has_row: true,
            has_col: true,
            has_row_end: false,
            has_col_end: true,
        },
        TestLink {
            link: "\"foo\", lines 339-341",
            prefix: Some("\""),
            suffix: Some("\", lines 339-341"),
            has_row: true,
            has_col: false,
            has_row_end: true,
            has_col_end: false,
        },
        TestLink {
            link: "\"foo\", lines 339-341, characters 12-789",
            prefix: Some("\""),
            suffix: Some("\", lines 339-341, characters 12-789"),
            has_row: true,
            has_col: true,
            has_row_end: true,
            has_col_end: true,
        },
        // Non-breaking space
        TestLink {
            link: "foo\u{00A0}339:12",
            prefix: None,
            suffix: Some("\u{00A0}339:12"),
            has_row: true,
            has_col: true,
            has_row_end: false,
            has_col_end: false,
        },
        TestLink {
            link: "\"foo\" on line 339,\u{00A0}column 12",
            prefix: Some("\""),
            suffix: Some("\" on line 339,\u{00A0}column 12"),
            has_row: true,
            has_col: true,
            has_row_end: false,
            has_col_end: false,
        },
        TestLink {
            link: "'foo' on line\u{00A0}339, column 12",
            prefix: Some("'"),
            suffix: Some("' on line\u{00A0}339, column 12"),
            has_row: true,
            has_col: true,
            has_row_end: false,
            has_col_end: false,
        },
        TestLink {
            link: "foo (339,\u{00A0}12)",
            prefix: None,
            suffix: Some(" (339,\u{00A0}12)"),
            has_row: true,
            has_col: true,
            has_row_end: false,
            has_col_end: false,
        },
        TestLink {
            link: "foo\u{00A0}[339, 12]",
            prefix: None,
            suffix: Some("\u{00A0}[339, 12]"),
            has_row: true,
            has_col: true,
            has_row_end: false,
            has_col_end: false,
        },
    ];

    fn get_test_links_with_suffix() -> Vec<&'static TestLink> {
        TEST_LINKS.iter().filter(|l| l.suffix.is_some()).collect()
    }

    #[test]
    fn test_remove_link_suffix() {
        for test_link in TEST_LINKS {
            let result = remove_link_suffix(test_link.link);
            let expected = if let Some(suffix) = test_link.suffix {
                test_link.link.replace(suffix, "")
            } else {
                test_link.link.to_string()
            };
            assert_eq!(result, expected);
        }
    }

    #[test]
    fn test_get_link_suffix() {
        for test_link in TEST_LINKS {
            let result = get_link_suffix(test_link.link);

            if let Some(suffix) = test_link.suffix {
                let expected = LinkSuffix {
                    row: if test_link.has_row {
                        Some(TEST_ROW)
                    } else {
                        None
                    },
                    col: if test_link.has_col {
                        Some(TEST_COL)
                    } else {
                        None
                    },
                    row_end: if test_link.has_row_end {
                        Some(TEST_ROW_END)
                    } else {
                        None
                    },
                    col_end: if test_link.has_col_end {
                        Some(TEST_COL_END)
                    } else {
                        None
                    },
                    suffix: LinkPartialRange {
                        index: test_link.link.len() - suffix.len(),
                        text: suffix.to_string(),
                    },
                };

                assert_eq!(result, Some(expected));
            } else {
                assert_eq!(result, None);
            }
        }
    }

    #[test]
    fn test_detect_link_suffixes() {
        for test_link in TEST_LINKS {
            let result = detect_link_suffixes(test_link.link);

            if let Some(suffix) = test_link.suffix {
                let expected = vec![LinkSuffix {
                    row: if test_link.has_row {
                        Some(TEST_ROW)
                    } else {
                        None
                    },
                    col: if test_link.has_col {
                        Some(TEST_COL)
                    } else {
                        None
                    },
                    row_end: if test_link.has_row_end {
                        Some(TEST_ROW_END)
                    } else {
                        None
                    },
                    col_end: if test_link.has_col_end {
                        Some(TEST_COL_END)
                    } else {
                        None
                    },
                    suffix: LinkPartialRange {
                        index: test_link.link.len() - suffix.len(),
                        text: suffix.to_string(),
                    },
                }];

                assert_eq!(result, expected);
            } else {
                assert_eq!(result, Vec::new());
            }
        }
    }

    #[test]
    fn test_detect_link_suffixes_multiple() {
        let line = "foo(1, 2) bar[3, 4] baz on line 5";
        let result = detect_link_suffixes(line);

        let expected = vec![
            LinkSuffix {
                row: Some(1),
                col: Some(2),
                row_end: None,
                col_end: None,
                suffix: LinkPartialRange {
                    index: 3,
                    text: "(1, 2)".to_string(),
                },
            },
            LinkSuffix {
                row: Some(3),
                col: Some(4),
                row_end: None,
                col_end: None,
                suffix: LinkPartialRange {
                    index: 13,
                    text: "[3, 4]".to_string(),
                },
            },
            LinkSuffix {
                row: Some(5),
                col: None,
                row_end: None,
                col_end: None,
                suffix: LinkPartialRange {
                    index: 23,
                    text: " on line 5".to_string(),
                },
            },
        ];

        assert_eq!(result, expected);
    }

    #[test]
    fn test_remove_link_query_string() {
        let test_cases = vec![
            ("?a=b", ""),
            ("foo?a=b", "foo"),
            ("./foo?a=b", "./foo"),
            ("/foo/bar?a=b", "/foo/bar"),
            ("foo?a=b?", "foo"),
            ("foo?a=b&c=d", "foo"),
            ("\\\\?\\foo?a=b", "\\\\?\\foo"),
        ];

        for (input, expected) in test_cases {
            assert_eq!(remove_link_query_string(input), expected);
        }
    }

    #[test]
    fn test_detect_links() {
        let line = "foo(1, 2) bar[3, 4] \"baz\" on line 5";
        let results = detect_links(line, OperatingSystem::Linux);

        let expected = vec![
            crate::ParsedLink {
                path: LinkPartialRange {
                    index: 0,
                    text: "foo".to_string(),
                },
                prefix: None,
                suffix: Some(LinkSuffix {
                    row: Some(1),
                    col: Some(2),
                    row_end: None,
                    col_end: None,
                    suffix: LinkPartialRange {
                        index: 3,
                        text: "(1, 2)".to_string(),
                    },
                }),
            },
            crate::ParsedLink {
                path: LinkPartialRange {
                    index: 10,
                    text: "bar".to_string(),
                },
                prefix: None,
                suffix: Some(LinkSuffix {
                    row: Some(3),
                    col: Some(4),
                    row_end: None,
                    col_end: None,
                    suffix: LinkPartialRange {
                        index: 13,
                        text: "[3, 4]".to_string(),
                    },
                }),
            },
            crate::ParsedLink {
                path: LinkPartialRange {
                    index: 21,
                    text: "baz".to_string(),
                },
                prefix: Some(LinkPartialRange {
                    index: 20,
                    text: "\"".to_string(),
                }),
                suffix: Some(LinkSuffix {
                    row: Some(5),
                    col: None,
                    row_end: None,
                    col_end: None,
                    suffix: LinkPartialRange {
                        index: 24,
                        text: "\" on line 5".to_string(),
                    },
                }),
            },
        ];

        assert_eq!(results, expected);
    }

    #[test]
    fn test_detect_links_with_prefix() {
        let line = r#""foo", line 5, col 6"#;
        let results = detect_links(line, OperatingSystem::Linux);

        let expected = vec![crate::ParsedLink {
            path: LinkPartialRange {
                index: 1,
                text: "foo".to_string(),
            },
            prefix: Some(LinkPartialRange {
                index: 0,
                text: "\"".to_string(),
            }),
            suffix: Some(LinkSuffix {
                row: Some(5),
                col: Some(6),
                row_end: None,
                col_end: None,
                suffix: LinkPartialRange {
                    index: 4,
                    text: "\", line 5, col 6".to_string(),
                },
            }),
        }];

        assert_eq!(expected, results);
    }

    #[test]
    fn test_detect_links_with_nested_quotes() {
        let line = "echo '\"foo\", line 5, col 6'";
        let results = detect_links(line, OperatingSystem::Linux);

        let expected = vec![crate::ParsedLink {
            path: LinkPartialRange {
                index: 7,
                text: "foo".to_string(),
            },
            prefix: Some(LinkPartialRange {
                index: 6,
                text: "\"".to_string(),
            }),
            suffix: Some(LinkSuffix {
                row: Some(5),
                col: Some(6),
                row_end: None,
                col_end: None,
                suffix: LinkPartialRange {
                    index: 10,
                    text: "\", line 5, col 6".to_string(),
                },
            }),
        }];

        assert_eq!(results, expected);
    }

    #[test]
    fn test_detect_links_multiple_types() {
        let line = "PS C:\\Github\\microsoft\\vscode> echo '\"foo\", line 5, col 6'";
        let results = detect_links(line, OperatingSystem::Windows);

        let expected = vec![
            crate::ParsedLink {
                path: LinkPartialRange {
                    index: 3,
                    text: "C:\\Github\\microsoft\\vscode".to_string(),
                },
                prefix: None,
                suffix: None,
            },
            crate::ParsedLink {
                path: LinkPartialRange {
                    index: 38,
                    text: "foo".to_string(),
                },
                prefix: Some(LinkPartialRange {
                    index: 37,
                    text: "\"".to_string(),
                }),
                suffix: Some(LinkSuffix {
                    row: Some(5),
                    col: Some(6),
                    row_end: None,
                    col_end: None,
                    suffix: LinkPartialRange {
                        index: 41,
                        text: "\", line 5, col 6".to_string(),
                    },
                }),
            },
        ];

        assert_eq!(results, expected);
    }

    #[test]
    fn test_detect_links_with_pipe_chars() {
        // Test with pipe characters
        let line = "|C:\\Github\\microsoft\\vscode|";
        let results = detect_links(line, OperatingSystem::Windows);

        let expected = vec![crate::ParsedLink {
            path: LinkPartialRange {
                index: 1,
                text: "C:\\Github\\microsoft\\vscode".to_string(),
            },
            prefix: None,
            suffix: None,
        }];

        assert_eq!(results, expected);

        // Test with pipe characters and suffix
        let line = "|C:\\Github\\microsoft\\vscode:400|";
        let results = detect_links(line, OperatingSystem::Windows);

        let expected = vec![crate::ParsedLink {
            path: LinkPartialRange {
                index: 1,
                text: "C:\\Github\\microsoft\\vscode".to_string(),
            },
            prefix: None,
            suffix: Some(LinkSuffix {
                row: Some(400),
                col: None,
                row_end: None,
                col_end: None,
                suffix: LinkPartialRange {
                    index: 27,
                    text: ":400".to_string(),
                },
            }),
        }];

        assert_eq!(results, expected);
    }

    #[test]
    fn test_test_detect_links_with_angle_brackets() {
        // Test for each operating system
        for (idx, os) in [
            OperatingSystem::Linux,
            OperatingSystem::Macintosh,
            OperatingSystem::Windows,
        ]
        .iter()
        .enumerate()
        {
            let path = OS_TEST_PATHS[idx];

            // Test with angle brackets
            let line = format!("<{}<", path);
            let results = detect_links(&line, *os);

            let expected = vec![crate::ParsedLink {
                path: LinkPartialRange {
                    index: 1,
                    text: path.to_string(),
                },
                prefix: None,
                suffix: None,
            }];

            assert_eq!(results, expected);

            // Test with angle brackets and suffix
            let line = format!("<{}:400<", path);
            let results = detect_links(&line, *os);

            let expected = vec![crate::ParsedLink {
                path: LinkPartialRange {
                    index: 1,
                    text: path.to_string(),
                },
                prefix: None,
                suffix: Some(LinkSuffix {
                    row: Some(400),
                    col: None,
                    row_end: None,
                    col_end: None,
                    suffix: LinkPartialRange {
                        index: 1 + path.len(),
                        text: ":400".to_string(),
                    },
                }),
            }];

            assert_eq!(results, expected);

            // Test with angle brackets (reversed)
            let line = format!(">{}>", path);
            let results = detect_links(&line, *os);

            let expected = vec![crate::ParsedLink {
                path: LinkPartialRange {
                    index: 1,
                    text: path.to_string(),
                },
                prefix: None,
                suffix: None,
            }];

            assert_eq!(results, expected);

            // Test with angle brackets and suffix (reversed)
            let line = format!(">{}:400>", path);
            let results = detect_links(&line, *os);

            let expected = vec![crate::ParsedLink {
                path: LinkPartialRange {
                    index: 1,
                    text: path.to_string(),
                },
                prefix: None,
                suffix: Some(LinkSuffix {
                    row: Some(400),
                    col: None,
                    row_end: None,
                    col_end: None,
                    suffix: LinkPartialRange {
                        index: 1 + path.len(),
                        text: ":400".to_string(),
                    },
                }),
            }];

            assert_eq!(results, expected);
        }
    }

    #[test]
    fn test_detect_links_with_query_strings() {
        // Test for each operating system
        for (idx, os) in [
            OperatingSystem::Linux,
            OperatingSystem::Macintosh,
            OperatingSystem::Windows,
        ]
        .iter()
        .enumerate()
        {
            let path = OS_TEST_PATHS[idx];

            // Test with query string
            let line = format!("{}?a=b", path);
            let results = detect_links(&line, *os);

            let expected = vec![crate::ParsedLink {
                path: LinkPartialRange {
                    index: 0,
                    text: path.to_string(),
                },
                prefix: None,
                suffix: None,
            }];

            assert_eq!(results, expected);

            // Test with more complex query string
            let line = format!("{}?a=b&c=d", path);
            let results = detect_links(&line, *os);

            let expected = vec![crate::ParsedLink {
                path: LinkPartialRange {
                    index: 0,
                    text: path.to_string(),
                },
                prefix: None,
                suffix: None,
            }];

            assert_eq!(results, expected);

            // Test no links starting with ? in query strings
            let line = "http://foo.com/?bar=/a/b&baz=c";
            let results = detect_links(&line, *os);
            assert!(!results.iter().any(|link| link.path.text.starts_with('?')));

            // Test no links starting with ? in Windows-style query strings
            let line = "http://foo.com/?bar=a:\\b&baz=c";
            let results = detect_links(&line, *os);
            assert!(!results.iter().any(|link| link.path.text.starts_with('?')));
        }
    }

    #[test]
    fn test_detect_links_in_git_diffs() {
        // Test for "--- a/foo/bar"
        let line = "--- a/foo/bar";
        let results = detect_links(line, OperatingSystem::Linux);

        let expected = vec![crate::ParsedLink {
            path: LinkPartialRange {
                index: 6,
                text: "foo/bar".to_string(),
            },
            prefix: None,
            suffix: None,
        }];

        assert_eq!(results, expected);

        // Test for "+++ b/foo/bar"
        let line = "+++ b/foo/bar";
        let results = detect_links(line, OperatingSystem::Linux);

        let expected = vec![crate::ParsedLink {
            path: LinkPartialRange {
                index: 6,
                text: "foo/bar".to_string(),
            },
            prefix: None,
            suffix: None,
        }];

        assert_eq!(results, expected);

        // Test for "diff --git a/foo/bar b/foo/baz"
        let line = "diff --git a/foo/bar b/foo/baz";
        let results = detect_links(line, OperatingSystem::Linux);

        let expected = vec![
            crate::ParsedLink {
                path: LinkPartialRange {
                    index: 13,
                    text: "foo/bar".to_string(),
                },
                prefix: None,
                suffix: None,
            },
            crate::ParsedLink {
                path: LinkPartialRange {
                    index: 23,
                    text: "foo/baz".to_string(),
                },
                prefix: None,
                suffix: None,
            },
        ];

        assert_eq!(results, expected);
    }

    #[test]
    fn test_detect_multiple_suffix_links() {
        let test_links_with_suffix = get_test_links_with_suffix();

        // Test combinations of three links with suffixes
        for i in 0..test_links_with_suffix.len().saturating_sub(2) {
            let link1 = test_links_with_suffix[i];
            let link2 = test_links_with_suffix[i + 1];
            let link3 = test_links_with_suffix[i + 2];

            let line = format!(" {} {} {} ", link1.link, link2.link, link3.link);
            let results = detect_links(&line, OperatingSystem::Linux);

            // Calculate start indices for each link
            let idx1 = 1;
            let idx2 = idx1 + link1.link.len() + 1;
            let idx3 = idx2 + link2.link.len() + 1;

            let expected = vec![
                crate::ParsedLink {
                    prefix: link1.prefix.map(|p| LinkPartialRange {
                        index: idx1,
                        text: p.to_string(),
                    }),
                    path: LinkPartialRange {
                        index: idx1 + (link1.prefix.map_or(0, |p| p.len())),
                        text: link1
                            .link
                            .replace(link1.suffix.unwrap(), "")
                            .replace(link1.prefix.unwrap_or(""), ""),
                    },
                    suffix: Some(LinkSuffix {
                        row: if link1.has_row { Some(TEST_ROW) } else { None },
                        col: if link1.has_col { Some(TEST_COL) } else { None },
                        row_end: if link1.has_row_end {
                            Some(TEST_ROW_END)
                        } else {
                            None
                        },
                        col_end: if link1.has_col_end {
                            Some(TEST_COL_END)
                        } else {
                            None
                        },
                        suffix: LinkPartialRange {
                            index: idx1 + (link1.link.len() - link1.suffix.unwrap().len()),
                            text: link1.suffix.unwrap().to_string(),
                        },
                    }),
                },
                crate::ParsedLink {
                    prefix: link2.prefix.map(|p| LinkPartialRange {
                        index: idx2,
                        text: p.to_string(),
                    }),
                    path: LinkPartialRange {
                        index: idx2 + (link2.prefix.map_or(0, |p| p.len())),
                        text: link2
                            .link
                            .replace(link2.suffix.unwrap(), "")
                            .replace(link2.prefix.unwrap_or(""), ""),
                    },
                    suffix: Some(LinkSuffix {
                        row: if link2.has_row { Some(TEST_ROW) } else { None },
                        col: if link2.has_col { Some(TEST_COL) } else { None },
                        row_end: if link2.has_row_end {
                            Some(TEST_ROW_END)
                        } else {
                            None
                        },
                        col_end: if link2.has_col_end {
                            Some(TEST_COL_END)
                        } else {
                            None
                        },
                        suffix: LinkPartialRange {
                            index: idx2 + (link2.link.len() - link2.suffix.unwrap().len()),
                            text: link2.suffix.unwrap().to_string(),
                        },
                    }),
                },
                crate::ParsedLink {
                    prefix: link3.prefix.map(|p| LinkPartialRange {
                        index: idx3,
                        text: p.to_string(),
                    }),
                    path: LinkPartialRange {
                        index: idx3 + (link3.prefix.map_or(0, |p| p.len())),
                        text: link3
                            .link
                            .replace(link3.suffix.unwrap(), "")
                            .replace(link3.prefix.unwrap_or(""), ""),
                    },
                    suffix: Some(LinkSuffix {
                        row: if link3.has_row { Some(TEST_ROW) } else { None },
                        col: if link3.has_col { Some(TEST_COL) } else { None },
                        row_end: if link3.has_row_end {
                            Some(TEST_ROW_END)
                        } else {
                            None
                        },
                        col_end: if link3.has_col_end {
                            Some(TEST_COL_END)
                        } else {
                            None
                        },
                        suffix: LinkPartialRange {
                            index: idx3 + (link3.link.len() - link3.suffix.unwrap().len()),
                            text: link3.suffix.unwrap().to_string(),
                        },
                    }),
                },
            ];

            assert_eq!(results, expected);
        }
    }

    #[test]
    fn test_ignore_empty_path_links() {
        // Test links where path is empty string with suffixes
        let line = "\"\",1";
        let results = detect_links(line, OperatingSystem::Linux);
        assert_eq!(results, Vec::new());
    }
}
