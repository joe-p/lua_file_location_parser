#[cfg(test)]
mod link_parsing_tests {
    use crate::{
        LinkPartialRange, LinkSuffix, OperatingSystem, detect_link_suffixes, detect_links,
        get_link_suffix, remove_link_query_string, remove_link_suffix,
    };
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
            if let Some(suffix) = test_link.suffix {
                assert_eq!(result, test_link.link.replace(suffix, ""));
            } else {
                assert_eq!(result, test_link.link);
            }
        }
    }

    #[test]
    fn test_get_link_suffix() {
        for test_link in TEST_LINKS {
            let result = get_link_suffix(test_link.link);

            if let Some(suffix) = test_link.suffix {
                // Create expected object
                let expected_suffix = LinkPartialRange {
                    index: test_link.link.len() - suffix.len(),
                    text: suffix.to_string(),
                };

                let expected_row = if test_link.has_row {
                    Some(TEST_ROW)
                } else {
                    None
                };
                let expected_col = if test_link.has_col {
                    Some(TEST_COL)
                } else {
                    None
                };
                let expected_row_end = if test_link.has_row_end {
                    Some(TEST_ROW_END)
                } else {
                    None
                };
                let expected_col_end = if test_link.has_col_end {
                    Some(TEST_COL_END)
                } else {
                    None
                };

                // Verify result
                assert!(
                    result.is_some(),
                    "Expected a suffix for link {}",
                    test_link.link
                );
                if let Some(actual) = result {
                    assert_eq!(actual.row, expected_row);
                    assert_eq!(actual.col, expected_col);
                    assert_eq!(actual.row_end, expected_row_end);
                    assert_eq!(actual.col_end, expected_col_end);
                    assert_eq!(actual.suffix.index, expected_suffix.index);
                    assert_eq!(actual.suffix.text, expected_suffix.text);
                }
            } else {
                assert!(result.is_none());
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

                assert_eq!(result.len(), expected.len());
                if !result.is_empty() {
                    assert_eq!(result[0].row, expected[0].row);
                    assert_eq!(result[0].col, expected[0].col);
                    assert_eq!(result[0].row_end, expected[0].row_end);
                    assert_eq!(result[0].col_end, expected[0].col_end);
                    assert_eq!(result[0].suffix.index, expected[0].suffix.index);
                    assert_eq!(result[0].suffix.text, expected[0].suffix.text);
                }
            } else {
                assert!(result.is_empty());
            }
        }
    }

    #[test]
    fn test_detect_link_suffixes_multiple() {
        let line = "foo(1, 2) bar[3, 4] baz on line 5";
        let result = detect_link_suffixes(line);

        assert_eq!(result.len(), 3);

        // Verify first suffix
        assert_eq!(result[0].row, Some(1));
        assert_eq!(result[0].col, Some(2));
        assert_eq!(result[0].row_end, None);
        assert_eq!(result[0].col_end, None);
        assert_eq!(result[0].suffix.index, 3);
        assert_eq!(result[0].suffix.text, "(1, 2)");

        // Verify second suffix
        assert_eq!(result[1].row, Some(3));
        assert_eq!(result[1].col, Some(4));
        assert_eq!(result[1].row_end, None);
        assert_eq!(result[1].col_end, None);
        assert_eq!(result[1].suffix.index, 13);
        assert_eq!(result[1].suffix.text, "[3, 4]");

        // Verify third suffix
        assert_eq!(result[2].row, Some(5));
        assert_eq!(result[2].col, None);
        assert_eq!(result[2].row_end, None);
        assert_eq!(result[2].col_end, None);
        assert_eq!(result[2].suffix.index, 23);
        assert_eq!(result[2].suffix.text, " on line 5");
    }

    #[test]
    fn test_remove_link_query_string() {
        assert_eq!(remove_link_query_string("?a=b"), "");
        assert_eq!(remove_link_query_string("foo?a=b"), "foo");
        assert_eq!(remove_link_query_string("./foo?a=b"), "./foo");
        assert_eq!(remove_link_query_string("/foo/bar?a=b"), "/foo/bar");
        assert_eq!(remove_link_query_string("foo?a=b?"), "foo");
        assert_eq!(remove_link_query_string("foo?a=b&c=d"), "foo");

        // UNC paths
        assert_eq!(remove_link_query_string("\\\\?\\foo?a=b"), "\\\\?\\foo");
    }

    #[test]
    fn test_detect_links() {
        let line = "foo(1, 2) bar[3, 4] \"baz\" on line 5";
        let results = detect_links(line, OperatingSystem::Linux);

        assert_eq!(results.len(), 3);

        // First link
        assert_eq!(results[0].path.index, 0);
        assert_eq!(results[0].path.text, "foo");
        assert!(results[0].prefix.is_none());
        assert!(results[0].suffix.is_some());
        let suffix = results[0].suffix.as_ref().unwrap();
        assert_eq!(suffix.row, Some(1));
        assert_eq!(suffix.col, Some(2));
        assert_eq!(suffix.row_end, None);
        assert_eq!(suffix.col_end, None);
        assert_eq!(suffix.suffix.index, 3);
        assert_eq!(suffix.suffix.text, "(1, 2)");

        // Second link
        assert_eq!(results[1].path.index, 10);
        assert_eq!(results[1].path.text, "bar");
        assert!(results[1].prefix.is_none());
        assert!(results[1].suffix.is_some());
        let suffix = results[1].suffix.as_ref().unwrap();
        assert_eq!(suffix.row, Some(3));
        assert_eq!(suffix.col, Some(4));
        assert_eq!(suffix.row_end, None);
        assert_eq!(suffix.col_end, None);
        assert_eq!(suffix.suffix.index, 13);
        assert_eq!(suffix.suffix.text, "[3, 4]");

        // Third link
        assert_eq!(results[2].path.index, 21);
        assert_eq!(results[2].path.text, "baz");
        assert!(results[2].prefix.is_some());
        let prefix = results[2].prefix.as_ref().unwrap();
        assert_eq!(prefix.index, 20);
        assert_eq!(prefix.text, "\"");
        assert!(results[2].suffix.is_some());
        let suffix = results[2].suffix.as_ref().unwrap();
        assert_eq!(suffix.row, Some(5));
        assert_eq!(suffix.col, None);
        assert_eq!(suffix.row_end, None);
        assert_eq!(suffix.col_end, None);
        assert_eq!(suffix.suffix.index, 24);
        assert_eq!(suffix.suffix.text, "\" on line 5");
    }

    #[test]
    fn test_detect_links_with_prefix() {
        let line = r#"'"foo", line 5, col 6'"#;
        let results = detect_links(line, OperatingSystem::Linux);

        assert_eq!(results.len(), 1);

        assert_eq!(results[0].path.index, 1);
        assert_eq!(results[0].path.text, "foo");
        assert!(results[0].prefix.is_some());
        let prefix = results[0].prefix.as_ref().unwrap();
        assert_eq!(prefix.index, 0);
        assert_eq!(prefix.text, "\"");
        assert!(results[0].suffix.is_some());
        let suffix = results[0].suffix.as_ref().unwrap();
        assert_eq!(suffix.row, Some(5));
        assert_eq!(suffix.col, Some(6));
        assert_eq!(suffix.row_end, None);
        assert_eq!(suffix.col_end, None);
        assert_eq!(suffix.suffix.index, 4);
        assert_eq!(suffix.suffix.text, "\", line 5, col 6");
    }

    #[test]
    fn test_detect_links_with_nested_quotes() {
        let line = "echo '\"foo\", line 5, col 6'";
        let results = detect_links(line, OperatingSystem::Linux);

        println!("{:#?}", results);
        assert_eq!(results.len(), 1);

        assert_eq!(results[0].path.index, 7);
        assert_eq!(results[0].path.text, "foo");
        assert!(results[0].prefix.is_some());
        let prefix = results[0].prefix.as_ref().unwrap();
        assert_eq!(prefix.index, 6);
        assert_eq!(prefix.text, "\"");
        assert!(results[0].suffix.is_some());
        let suffix = results[0].suffix.as_ref().unwrap();
        assert_eq!(suffix.row, Some(5));
        assert_eq!(suffix.col, Some(6));
        assert_eq!(suffix.row_end, None);
        assert_eq!(suffix.col_end, None);
        assert_eq!(suffix.suffix.index, 10);
        assert_eq!(suffix.suffix.text, "\", line 5, col 6");
    }

    #[test]
    fn test_detect_links_multiple_types() {
        let line = "PS C:\\Github\\microsoft\\vscode> echo '\"foo\", line 5, col 6'";
        let results = detect_links(line, OperatingSystem::Windows);

        assert_eq!(results.len(), 2);

        // First link (path without suffix)
        assert_eq!(results[0].path.index, 3);
        assert_eq!(results[0].path.text, "C:\\Github\\microsoft\\vscode");
        assert!(results[0].prefix.is_none());
        assert!(results[0].suffix.is_none());

        // Second link (with prefix and suffix)
        assert_eq!(results[1].path.index, 38);
        assert_eq!(results[1].path.text, "foo");
        assert!(results[1].prefix.is_some());
        let prefix = results[1].prefix.as_ref().unwrap();
        assert_eq!(prefix.index, 37);
        assert_eq!(prefix.text, "\"");
        assert!(results[1].suffix.is_some());
        let suffix = results[1].suffix.as_ref().unwrap();
        assert_eq!(suffix.row, Some(5));
        assert_eq!(suffix.col, Some(6));
        assert_eq!(suffix.row_end, None);
        assert_eq!(suffix.col_end, None);
        assert_eq!(suffix.suffix.index, 41);
        assert_eq!(suffix.suffix.text, "\", line 5, col 6");
    }

    #[test]
    fn test_detect_links_with_pipe_chars() {
        // Test with pipe characters
        let line = "|C:\\Github\\microsoft\\vscode|";
        let results = detect_links(line, OperatingSystem::Windows);

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].path.index, 1);
        assert_eq!(results[0].path.text, "C:\\Github\\microsoft\\vscode");
        assert!(results[0].prefix.is_none());
        assert!(results[0].suffix.is_none());

        // Test with pipe characters and suffix
        let line = "|C:\\Github\\microsoft\\vscode:400|";
        let results = detect_links(line, OperatingSystem::Windows);

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].path.index, 1);
        assert_eq!(results[0].path.text, "C:\\Github\\microsoft\\vscode");
        assert!(results[0].prefix.is_none());
        assert!(results[0].suffix.is_some());
        let suffix = results[0].suffix.as_ref().unwrap();
        assert_eq!(suffix.row, Some(400));
        assert_eq!(suffix.col, None);
        assert_eq!(suffix.row_end, None);
        assert_eq!(suffix.col_end, None);
        assert_eq!(suffix.suffix.index, 27);
        assert_eq!(suffix.suffix.text, ":400");
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

            assert_eq!(results.len(), 1);
            assert_eq!(results[0].path.index, 1);
            assert_eq!(results[0].path.text, path);
            assert!(results[0].prefix.is_none());
            assert!(results[0].suffix.is_none());

            // Test with angle brackets and suffix
            let line = format!("<{}:400<", path);
            let results = detect_links(&line, *os);

            assert_eq!(results.len(), 1);
            assert_eq!(results[0].path.index, 1);
            assert_eq!(results[0].path.text, path);
            assert!(results[0].prefix.is_none());
            assert!(results[0].suffix.is_some());
            let suffix = results[0].suffix.as_ref().unwrap();
            assert_eq!(suffix.row, Some(400));
            assert_eq!(suffix.col, None);
            assert_eq!(suffix.row_end, None);
            assert_eq!(suffix.col_end, None);
            assert_eq!(suffix.suffix.index, 1 + path.len());
            assert_eq!(suffix.suffix.text, ":400");

            // Test with angle brackets (reversed)
            let line = format!(">{}>", path);
            let results = detect_links(&line, *os);

            assert_eq!(results.len(), 1);
            assert_eq!(results[0].path.index, 1);
            assert_eq!(results[0].path.text, path);
            assert!(results[0].prefix.is_none());
            assert!(results[0].suffix.is_none());

            // Test with angle brackets and suffix (reversed)
            let line = format!(">{}:400>", path);
            let results = detect_links(&line, *os);

            assert_eq!(results.len(), 1);
            assert_eq!(results[0].path.index, 1);
            assert_eq!(results[0].path.text, path);
            assert!(results[0].prefix.is_none());
            assert!(results[0].suffix.is_some());
            let suffix = results[0].suffix.as_ref().unwrap();
            assert_eq!(suffix.row, Some(400));
            assert_eq!(suffix.col, None);
            assert_eq!(suffix.row_end, None);
            assert_eq!(suffix.col_end, None);
            assert_eq!(suffix.suffix.index, 1 + path.len());
            assert_eq!(suffix.suffix.text, ":400");
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

            assert_eq!(results.len(), 1);
            assert_eq!(results[0].path.index, 0);
            assert_eq!(results[0].path.text, path);
            assert!(results[0].prefix.is_none());
            assert!(results[0].suffix.is_none());

            // Test with more complex query string
            let line = format!("{}?a=b&c=d", path);
            let results = detect_links(&line, *os);

            assert_eq!(results.len(), 1);
            assert_eq!(results[0].path.index, 0);
            assert_eq!(results[0].path.text, path);
            assert!(results[0].prefix.is_none());
            assert!(results[0].suffix.is_none());

            // Test no links starting with ? in query strings
            let line = format!("http://foo.com/?bar=/a/b&baz=c");
            let results = detect_links(&line, *os);
            assert!(!results.iter().any(|link| link.path.text.starts_with('?')));

            // Test no links starting with ? in Windows-style query strings
            let line = format!("http://foo.com/?bar=a:\\b&baz=c");
            let results = detect_links(&line, *os);
            assert!(!results.iter().any(|link| link.path.text.starts_with('?')));
        }
    }

    #[test]
    fn test_detect_links_in_git_diffs() {
        // Test for "--- a/foo/bar"
        let line = "--- a/foo/bar";
        let results = detect_links(line, OperatingSystem::Linux);

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].path.index, 6);
        assert_eq!(results[0].path.text, "foo/bar");
        assert!(results[0].prefix.is_none());
        assert!(results[0].suffix.is_none());

        // Test for "+++ b/foo/bar"
        let line = "+++ b/foo/bar";
        let results = detect_links(line, OperatingSystem::Linux);

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].path.index, 6);
        assert_eq!(results[0].path.text, "foo/bar");
        assert!(results[0].prefix.is_none());
        assert!(results[0].suffix.is_none());

        // Test for "diff --git a/foo/bar b/foo/baz"
        let line = "diff --git a/foo/bar b/foo/baz";
        let results = detect_links(line, OperatingSystem::Linux);

        assert_eq!(results.len(), 2);
        assert_eq!(results[0].path.index, 13);
        assert_eq!(results[0].path.text, "foo/bar");
        assert!(results[0].prefix.is_none());
        assert!(results[0].suffix.is_none());

        assert_eq!(results[1].path.index, 23);
        assert_eq!(results[1].path.text, "foo/baz");
        assert!(results[1].prefix.is_none());
        assert!(results[1].suffix.is_none());
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

            assert_eq!(results.len(), 3, "Failed on line: {}", line);

            // Check first link
            let suffix1 = link1.suffix.unwrap();
            assert_eq!(
                results[0].path.index,
                1 + (link1.prefix.map_or(0, |p| p.len()))
            );
            assert_eq!(
                results[0].path.text,
                link1
                    .link
                    .replace(suffix1, "")
                    .replace(link1.prefix.unwrap_or(""), "")
            );

            if let Some(prefix) = link1.prefix {
                assert!(results[0].prefix.is_some());
                let prefix_obj = results[0].prefix.as_ref().unwrap();
                assert_eq!(prefix_obj.index, 1);
                assert_eq!(prefix_obj.text, prefix);
            } else {
                assert!(results[0].prefix.is_none());
            }

            assert!(results[0].suffix.is_some());
            let suffix_obj = results[0].suffix.as_ref().unwrap();
            assert_eq!(suffix_obj.row.is_some(), link1.has_row);
            if link1.has_row {
                assert_eq!(suffix_obj.row, Some(TEST_ROW));
            }
            assert_eq!(suffix_obj.col.is_some(), link1.has_col);
            if link1.has_col {
                assert_eq!(suffix_obj.col, Some(TEST_COL));
            }
            assert_eq!(suffix_obj.row_end.is_some(), link1.has_row_end);
            if link1.has_row_end {
                assert_eq!(suffix_obj.row_end, Some(TEST_ROW_END));
            }
            assert_eq!(suffix_obj.col_end.is_some(), link1.has_col_end);
            if link1.has_col_end {
                assert_eq!(suffix_obj.col_end, Some(TEST_COL_END));
            }
            assert_eq!(
                suffix_obj.suffix.index,
                1 + (link1.link.len() - suffix1.len())
            );
            assert_eq!(suffix_obj.suffix.text, suffix1);

            // Spot check other links
            assert!(results[1].path.index > results[0].path.index);
            assert!(results[2].path.index > results[1].path.index);
        }
    }

    #[test]
    fn test_ignore_empty_path_links() {
        // Test links where path is empty string with suffixes
        let line = "\"\",1";
        let results = detect_links(line, OperatingSystem::Linux);
        assert_eq!(results.len(), 0, "Empty path links should be ignored");
    }
}
