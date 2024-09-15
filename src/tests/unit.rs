use crate::*;

#[cfg(test)]

#[test]
fn test_create_patterns_filters_single_line() {
    let arg = "--patterns=*.rs,!test/*.rs";
    let filters = create_patterns_filters(arg);
    assert_eq!(filters.len(), 2);
    assert_eq!(filters[0].pattern, "*.rs");
    assert_eq!(
        filters[0].exclude, false,
        "Expected 'exclude' to be false for pattern '*.rs'"
    );
    assert_eq!(filters[1].pattern, "test/*.rs");
    assert_eq!(filters[1].exclude, true);
}

#[test]
fn test_create_patterns_filters_multiple_lines() {
    let arg = "--patterns=*.rs
    !test/*.rs
    .gitignore
    ";
    let filters = create_patterns_filters(arg);

    assert_eq!(filters.len(), 3);
    assert_eq!(filters[0].pattern, "*.rs");
    assert_eq!(filters[0].exclude, false);
    assert_eq!(filters[1].pattern, "test/*.rs");
    assert_eq!(filters[1].exclude, true);
    assert_eq!(filters[2].pattern, ".gitignore");
    assert_eq!(filters[2].exclude, false);
}

#[test]
fn test_categorize_filters() {
    let filters = vec![
        PatternFilter {
            pattern: String::from("*.rs"),
            exclude: false,
        },
        PatternFilter {
            pattern: String::from("test/*.rs"),
            exclude: true,
        },
    ];
    let (include_patterns_filters, exclude_patterns_filters) = categorize_filters(filters);
    assert_eq!(include_patterns_filters.len(), 1);
    assert_eq!(exclude_patterns_filters.len(), 1);
    assert_eq!(include_patterns_filters.contains("*.rs"), true);
    assert_eq!(exclude_patterns_filters.contains("test/*.rs"), true);
}

#[test]
fn test_filter() {
    let files = vec![
        String::from("src/main.rs"),
        String::from("lib.rs"),
        String::from("test.txt"),
    ];
    let include_patterns_filters = HashSet::from([
        String::from("*.rs"),
        String::from("src/**"),
        String::from("*.txt"),
    ]);
    let exclude_patterns_filters = HashSet::from([
        String::from("test.txt")
    ]);
    let filtered_files = filter_files(files, include_patterns_filters, exclude_patterns_filters);
    let expected_filtered_files = HashSet::from([
        String::from("src/main.rs"),
        String::from("lib.rs"),
    ]);

    assert_eq!(filtered_files, expected_filtered_files);
}

#[test]
fn test_filter_exclude_files_exclusion() {
    let exclude_patterns_filters = HashSet::from([
        String::from("test.txt"),
    ]);
    let include_patterns_filters = HashSet::from([
        String::from("*.rs"),
        String::from("*.txt"),
    ]);

    let files = vec![
        String::from("main.rs"),
        String::from("lib.rs"),
        String::from("version.txt"),
        String::from("test.txt"),
    ];

    let filtered_files = filter_files(files, include_patterns_filters, exclude_patterns_filters);
    let expected_filtered_files = HashSet::from([
        String::from("main.rs"),
        String::from("lib.rs"),
        String::from("version.txt"),
    ]);

    assert_eq!(filtered_files, expected_filtered_files);
}

#[test]
fn test_get_count() {
    let files = HashSet::from([
        String::from("main.rs"),
        String::from("lib.rs"),
        String::from("version.txt"),
    ]);
    let count = get_count(files);
    assert_eq!(count, 3);
}
