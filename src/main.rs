use clap::Parser;
use anyhow::{Context, Result};
use regex::Regex;
use std::collections::{HashSet, HashMap};

#[derive(Parser)]
struct Cli {
    // The file to look for
    path: std::path::PathBuf
}

fn main() -> Result<()>{
    let path = parse_file_path();
    let content = get_file_content(&path)?;
    let masks = find_masked_inputs(&content);
    prompt_user_inputs(&masks);
    
    Ok(())
}

fn parse_file_path() -> String {
    let args = Cli::parse();
    args.path.as_os_str().to_str().unwrap().to_string()
}

fn get_file_content(path : &String) -> Result<String> {
    println!("Opening the file {}", path);
    let content = std::fs::read_to_string(&path)
        .with_context(|| format!("could not read file `{}`", &path))?;
    println!("file content: {}", content);
    Ok(content)
}

fn find_masked_inputs(input: &str) -> HashSet<String> {
    let re = Regex::new(r"\{\{([^{}]+)\}\}").unwrap();
    let matches = re.find_iter(input);
    let contents: HashSet<String> = matches.map(|m| m.as_str().trim_matches(|c| c == '{' || c == '}' || c == ' ').to_owned()).collect();
    contents
}

fn prompt_user_inputs(masks : &HashSet<String>) {
    let mut map = HashMap::new();
    for input in masks {
        let value = prompt_single_input(input);
        map.insert(input.to_string(), value);
    }
}

fn prompt_single_input(input: &str) -> String {
    let mut value = String::new();
    println!("{}", input);
    std::io::stdin().read_line(&mut value).expect("Failed to read input");
    value.trim().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_masked_inputs_empty() {
        let input = "";
        let expected: HashSet<String> = HashSet::new();
        assert_eq!(find_masked_inputs(input), expected);
    }

    #[test]
    fn test_find_masked_inputs_no_match() {
        let input = "This is a test.";
        let expected: HashSet<String> = HashSet::new();
        assert_eq!(find_masked_inputs(input), expected);
    }

    #[test]
    fn test_find_masked_inputs_single_match() {
        let input = "This {{is}} a test.";
        let mut expected: HashSet<String> = HashSet::new();
        expected.insert(String::from("is"));
        assert_eq!(find_masked_inputs(input), expected);
    }

    #[test]
    fn test_find_masked_inputs_multiple_matches() {
        let input = "{{This}} {{is}} a {{test}} with {{many}} {{masked}} inputs.";
        let mut expected: HashSet<String> = HashSet::new();
        expected.insert(String::from("This"));
        expected.insert(String::from("is"));
        expected.insert(String::from("test"));
        expected.insert(String::from("many"));
        expected.insert(String::from("masked"));
        assert_eq!(find_masked_inputs(input), expected);
    }

    #[test]
    fn test_find_masked_inputs_duplicate_matches() {
        let input = "{{This}} is {{a}} {{test}} with {{many}} {{masked}} inputs. {{This}} {{is}} {{a}} {{test}} with {{many}} {{masked}} inputs.";
        let mut expected: HashSet<String> = HashSet::new();
        expected.insert(String::from("This"));
        expected.insert(String::from("a"));
        expected.insert(String::from("test"));
        expected.insert(String::from("many"));
        expected.insert(String::from("masked"));
        expected.insert(String::from("is"));
        assert_eq!(find_masked_inputs(input), expected);
    }
}


