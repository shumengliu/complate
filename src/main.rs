use anyhow::{Context, Result};
use clap::Parser;
use std::collections::{HashMap, HashSet, VecDeque};
mod utils;

#[derive(Parser)]
struct Cli {
    // The file to look for
    path: std::path::PathBuf,
}

// TODO refactor
// struct Placeholder {
//     mask: String
// }

fn main() -> Result<()> {
    let path = parse_file_path();
    let content = get_file_content(&path)?;
    let masks = find_placeholders(&content);
    let replacements = prompt_user_inputs(&masks);
    let result = replace_parts(&content, &replacements);
    println!("{}", &result);
    let _ = utils::set_clipboard_content(&result);
    Ok(())
}

fn parse_file_path() -> String {
    let args = Cli::parse();
    args.path.as_os_str().to_str().unwrap().to_string()
}

fn get_file_content(path: &String) -> Result<String> {
    println!("Opening the file {}", path);
    let content = std::fs::read_to_string(&path)
        .with_context(|| format!("could not read file `{}`", &path))?;
    println!("file content: {}", content);
    Ok(content)
}

/*
We want the results to be ordered by apperance AND unique.
Therefore we used a temporary Vec for ordering, and a helper HashSet to
remove duplicates.
*/
fn find_placeholders(input: &str) -> Vec<String> {
    let re = regex::Regex::new(r"\{\{([^{}]+)\}\}").unwrap();
    // get the masks
    let matches = re.find_iter(input);
    let contents = extract_contents(matches);
    remove_duplicates(contents)
}

// remove the outer brackets in the placeholders
fn extract_contents(matches: regex::Matches) -> VecDeque<String> {
    matches
        .map(|m| {
            m.as_str()
                .trim_matches(|c| c == '{' || c == '}' || c == ' ')
                .to_owned()
        })
        .collect()
}

fn remove_duplicates(contents: VecDeque<String>) -> Vec<String> {
    let mut result: Vec<String> = Vec::with_capacity(contents.len());
    let mut set: HashSet<String> = HashSet::new();
    for content in contents {
        if set.insert(content.clone()) {
            result.push(content);
        }
    }
    result
}

fn prompt_user_inputs(masks: &Vec<String>) -> HashMap<String, String> {
    let mut map = HashMap::new();
    for input in masks {
        let value = prompt_single_input(input);
        map.insert(input.to_string(), value);
    }
    println!("{:?}", map);
    map
}

fn prompt_single_input(placeholder: &str) -> String {
    let mut value = String::new();
    println!("What is the value for {}?", placeholder);
    std::io::stdin()
        .read_line(&mut value)
        .expect("Failed to read input");
    value.trim().to_string()
}

fn replace_parts(input: &str, replacements: &HashMap<String, String>) -> String {
    let mut output = input.to_string();
    for (from, to) in replacements {
        let from_temp = format!("{{{{{}}}}}", from);
        output = output.replace(&from_temp, to);
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_masked_inputs() {
        assert_eq!(
            find_placeholders("Hello {{world}}, {{world}}"),
            ["world".to_string()]
        );
        assert_eq!(
            find_placeholders("{{a}} b {{a}} c {{b}}"),
            ["a".to_string(), "b".to_string()]
        );
        assert_eq!(find_placeholders(""), Vec::<String>::new());
    }
}
