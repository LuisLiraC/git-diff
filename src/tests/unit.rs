use crate::*;

#[cfg(test)]
mod unit {
    use super::*;

    #[test]
    fn test_create_patterns_filters() {
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
        assert_eq!(include_patterns_filters[0].pattern, "*.rs");
        assert_eq!(exclude_patterns_filters[0].pattern, "test/*.rs");
    }

    #[test]
    fn test_filter() {
        let files = vec![
            String::from("src/main.rs"),
            String::from("lib.rs"),
            String::from("test.txt"),
        ];
        let include_patterns_filters = vec![
            PatternFilter {
                pattern: String::from("*.rs"),
                exclude: false,
            },
            PatternFilter {
                pattern: String::from("src/**"),
                exclude: false,
            },
            PatternFilter {
                pattern: String::from("*.txt"),
                exclude: false,
            },
        ];
        let exclude_patterns_filters = vec![PatternFilter {
            pattern: String::from("test.txt"),
            exclude: true,
        }];
        let filtered_files = filter(files, include_patterns_filters, exclude_patterns_filters);
        let expected_filtered_files = HashSet::from([
            String::from("src/main.rs"),
            String::from("lib.rs"),
        ]);

        assert_eq!(filtered_files, expected_filtered_files);
    }

    #[test]
    fn test_filter_files_by_pattern() {
        let pattern_filter = PatternFilter {
            pattern: String::from("*.rs"),
            exclude: false,
        };
        let files = vec![
            String::from("main.rs"),
            String::from("lib.rs"),
            String::from("test.txt"),
        ];
        let filtered = filter_files_by_pattern(&pattern_filter, &files, &Vec::new());
        assert_eq!(
            filtered,
            vec![String::from("main.rs"), String::from("lib.rs")]
        );
    }

    #[test]
    fn test_filter_exclude_files_exclusion() {
        let mut filtered_files: Vec<String> = Vec::new();
        let mut exclude_patterns_filters: Vec<PatternFilter> = Vec::new();
        let mut include_patterns_filters: Vec<PatternFilter> = Vec::new();

        include_patterns_filters.push(PatternFilter {
            pattern: String::from("*.rs"),
            exclude: false,
        });

        include_patterns_filters.push(PatternFilter {
            pattern: String::from("*.txt"),
            exclude: false,
        });

        exclude_patterns_filters.push(PatternFilter {
            pattern: String::from("test.txt"),
            exclude: true,
        });

        let files = vec![
            String::from("main.rs"),
            String::from("lib.rs"),
            String::from("version.txt"),
            String::from("test.txt"),
        ];

        for pattern in include_patterns_filters.iter() {
            filtered_files.extend(filter_files_by_pattern(&pattern, &files, &exclude_patterns_filters));
        }

        assert_eq!(
            filtered_files,
            vec![String::from("main.rs"), String::from("lib.rs"), String::from("version.txt")]
        );
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
}
