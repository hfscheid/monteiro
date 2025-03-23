use git2::Repository;

pub fn clone(repo_name: &str, repo_dir: &str) {
    let _ = match Repository::clone(repo_name, repo_dir) {
        Ok(repo) => repo,
        Err(e) => panic!("Error: {}", e),
    };
}
