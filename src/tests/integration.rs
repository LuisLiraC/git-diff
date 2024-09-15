use crate::*;

#[cfg(test)]

#[test]
fn test_filter() {
    let arg = "--patterns=*.rs,!*..txt";
    let files = vec![
        String::from("src/main.rs"),
        String::from("lib.rs"),
        String::from("test.txt"),
    ];

    let filters = create_patterns_filters(arg);

    let (include_patterns_filters, exclude_patterns_filters) = categorize_filters(&filters);

    let filtered_files = filter_files(&files, &include_patterns_filters, &exclude_patterns_filters);

    let count = get_count(&filtered_files);

    let expected_filtered_files = HashSet::from([
        String::from("src/main.rs"),
        String::from("lib.rs"),
    ]);

    assert_eq!(filtered_files, expected_filtered_files);
    assert_eq!(count, 2);
}
