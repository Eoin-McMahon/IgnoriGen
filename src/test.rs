#[cfg(test)]
mod test {
    use std::collections::HashMap;

    #[test]
    #[should_panic]
    fn http_get() {
        // make sure a valid url does not return nothing
        assert_ne!(blindfold::http_get("https://api.github.com/repos/toptal/gitignore/contents/templates?ref=master"), "");
        // make sure that an invalid url causes an error (ok because the repo url is hardcoded)
        panic!(blindfold::http_get("www.notarealsite/foo/bar"));
    }

    #[test]
    fn generate_gitignore_file() {
        // setup
        let mut map: HashMap<String, String> = HashMap::new(); 
        map.insert(String::from("rust"), String::from("https://raw.githubusercontent.com/toptal/gitignore/master/templates/Rust.gitignore"));
        let langs = vec!["rust"];
        let empty_lang = vec![""];
        let has_empty_lang = vec!["", "rust"];
        
        // add a single language and generate a gitignore for it
        assert_eq!(blindfold::generate_gitignore_file(langs, &map), "# RUST gitignore generated by Blindfold\n\n# Generated by Cargo\n# will have compiled files and executables\n/target/\n\n# Remove Cargo.lock from gitignore if creating an executable, leave it for libraries\n# More information here https://doc.rust-lang.org/cargo/guide/cargo-toml-vs-cargo-lock.html\nCargo.lock\n\n# These are backup files generated by rustfmt\n**/*.rs.bk\n\n\n");
        // empty vector should return an empty string
        assert_eq!(blindfold::generate_gitignore_file(empty_lang, &map), "");
        // if there is an empty language, it should return a gitignore for the valid languages
        assert_eq!(blindfold::generate_gitignore_file(has_empty_lang, &map), "# RUST gitignore generated by Blindfold\n\n# Generated by Cargo\n# will have compiled files and executables\n/target/\n\n# Remove Cargo.lock from gitignore if creating an executable, leave it for libraries\n# More information here https://doc.rust-lang.org/cargo/guide/cargo-toml-vs-cargo-lock.html\nCargo.lock\n\n# These are backup files generated by rustfmt\n**/*.rs.bk\n\n\n");
    }
    
    #[test]
    fn get_gitignore_file() {
        // setup
        let mut map: HashMap<String, String> = HashMap::new(); 
        map.insert(String::from("rust"), String::from("https://raw.githubusercontent.com/toptal/gitignore/master/templates/Rust.gitignore"));
        let language = "rust";
        
        // get raw gitignore from the github api
        assert_eq!(blindfold::get_ignore_file(&map, language), "# Generated by Cargo\n# will have compiled files and executables\n/target/\n\n# Remove Cargo.lock from gitignore if creating an executable, leave it for libraries\n# More information here https://doc.rust-lang.org/cargo/guide/cargo-toml-vs-cargo-lock.html\nCargo.lock\n\n# These are backup files generated by rustfmt\n**/*.rs.bk\n");
        // non existent language should return an empty string
        assert_eq!(blindfold::get_ignore_file(&map, ""), "");
    } 
    
    #[test]
    fn suggest_most_similar() {
        // setup
        let mut map: HashMap<String, String> = HashMap::new(); 
        map.insert(String::from("rust"), String::from("rust gitignore url"));
        map.insert(String::from("c++"), String::from("c++ gitignore url"));
        let language = "roost";

        // this function takes a y/n from standard input, so need to pass this as input to the
        // function
        let yes_input = b"y";
        let no_input = b"n";
       
        let mut output = Vec::new();
        let yes_answer = blindfold::suggest_most_similar(&yes_input[..], &mut output, &language, map.clone());
        let no_answer = blindfold::suggest_most_similar(&no_input[..], &mut output, &language, map);

        // most similar should be rust
        assert_eq!(yes_answer, Some(String::from("rust")));
        // if not accepted the most similar should be `None`
        assert_eq!(no_answer, None);

    }
}
