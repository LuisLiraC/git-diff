use git2::Repository;
use std::collections::HashSet;
use std::env;
use glob::Pattern;
use std::process::Command;
use std::time::Instant;

#[derive(Clone, Debug)]
struct PatternFilter {
    pattern: String,
    exclude: bool,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut patterns_filters: Vec<PatternFilter> = Vec::new();

    for arg in args.iter() {
        if arg.starts_with("--patterns=") {
            patterns_filters = create_patterns_filters(&arg);
        }
    }

    if patterns_filters.is_empty() {
        println!("No patterns provided");
        return;
    }

    let (include_patterns_filters, exclude_patterns_filters) = categorize_filters(patterns_filters);

    let start = Instant::now();
    let changed_files = get_changed_files();
    let duration = start.elapsed();
    println!("Getting changed files done in: {:?}", duration);

    println!("Changed files: {:?}", changed_files);

    let start = Instant::now();
    let filtered_files = filter(changed_files, include_patterns_filters, exclude_patterns_filters);
    let duration = start.elapsed();
    println!("Filtering files done in: {:?}", duration);

    let count = get_count(filtered_files.clone());

    println!("Filtered files: {:?}", filtered_files);
    println!("Count: {}", count);

    Command::new("sh")
        .arg("-c")
        .arg(format!("echo \"DIFF_FILES={:?}\" >> $GITHUB_OUTPUT", filtered_files))
        .output()
        .expect("Failed to execute DIFF_FILES command");

    Command::new("sh")
        .arg("-c")
        .arg(format!("echo \"DIFF_COUNT={}\" >> $GITHUB_OUTPUT", count))
        .output()
        .expect("Failed to execute DIFF_COUNT command");
}

fn create_patterns_filters(arg: &str) -> Vec<PatternFilter> {
    let binding = arg
        .split('=')
        .last()
        .expect("Failed to get patterns")
        .replace(" ", "")
        .replace("\n", ",")
        .replace("\r", "")
        .replace(",,", ",")
        .to_string();

    let patterns = binding
        .split(',')
        .collect::<Vec<&str>>();

    let mut patterns_filters: Vec<PatternFilter> = Vec::new();

    for pattern in patterns.iter() {
        let exclude = pattern.starts_with('!');
        let pattern = if exclude {
            pattern[1..].to_string()
        } else {
            pattern.to_string()
        };

        patterns_filters.push(PatternFilter {
            pattern,
            exclude,
        });
    }

    patterns_filters
}

fn get_changed_files() -> Vec<String> {
    let repository = Repository::open(".").expect("Failed to open repository");

    let head = repository.head().expect("Failed to get HEAD");
    let head_commit = head.peel_to_commit().expect("Failed to peel HEAD to commit");

    // Refers to base branch in case of pull request. For example: main
    let base_ref_env = env::var("GITHUB_BASE_REF").expect("Failed to get GITHUB_BASE_REF env variable");

    Command::new("sh")
        .arg("-c")
        .arg(format!("git fetch origin {}", base_ref_env))
        .output()
        .expect("Failed to execute fetch branch command");

    let base_ref_string = format!("refs/remotes/origin/{}", base_ref_env);
    let base_ref = repository.find_reference(&base_ref_string).expect("Failed to find default branch");
    let base_commit = base_ref.peel_to_commit().expect("Failed to peel default branch to commit");

    let diff = repository.diff_tree_to_tree(
        Some(&base_commit.tree().expect("Failed to get base tree")),
        Some(&head_commit.tree().expect("Failed to get HEAD tree")),
        None,
    ).expect("Failed to get diff");

    let mut changed_files = Vec::new();
    diff.foreach(
        &mut |delta, _| {
            if let Some(path) = delta.new_file().path() {
                changed_files.push(path.to_string_lossy().into_owned());
            }
            true
        },
        None,
        None,
        None,
    ).expect("Error while iterating over diff");

    changed_files
}

fn filter(changed_files: Vec<String>, include_patterns_filters: Vec<PatternFilter>, exclude_patterns_filters: Vec<PatternFilter>) -> HashSet<String> {
    let filtered_files: Vec<String> = include_patterns_filters
        .iter()
        .flat_map(|pattern| filter_files_by_pattern(pattern, &changed_files, &exclude_patterns_filters))
        .collect();

    let mut hash_set_filtered_files = HashSet::new();
    hash_set_filtered_files.extend(filtered_files.clone());

    hash_set_filtered_files
}

fn filter_files_by_pattern(pattern_filter: &PatternFilter, files: &Vec<String>, exclude_patterns: &Vec<PatternFilter>) -> Vec<String> {
    let pattern = Pattern::new(&pattern_filter.pattern).expect("Failed to create pattern");

    let mut filtered_files: Vec<String> = files
        .iter()
        .filter(|file| pattern.matches(file))
        .filter(|_| pattern_filter.exclude == false)
        .map(|file| file.to_string())
        .collect();

    for exclude_pattern in exclude_patterns.iter() {
        filtered_files = filtered_files
            .iter()
            .filter(|file| !Pattern::new(&exclude_pattern.pattern).expect("Failed to create pattern").matches(file))
            .map(|file| file.to_string())
            .collect();
    }

    filtered_files
}

fn get_count(filtered_files: HashSet<String>) -> usize {
    filtered_files.len()
}

fn categorize_filters(filters: Vec<PatternFilter>) -> (Vec<PatternFilter>, Vec<PatternFilter>) {
    let mut exclude_patterns_filters: Vec<PatternFilter> = Vec::new();
    let mut include_patterns_filters: Vec<PatternFilter> = Vec::new();

    filters.iter().for_each(|pattern_filter| {
        if pattern_filter.exclude {
            exclude_patterns_filters.push(pattern_filter.clone());
        } else {
            include_patterns_filters.push(pattern_filter.clone());
        }
    });

    (include_patterns_filters, exclude_patterns_filters)
}

#[cfg(test)]
mod tests {
    mod unit;
    mod integration;
}