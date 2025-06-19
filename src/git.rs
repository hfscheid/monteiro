// use git2::Repository;
use git2::{Cred, RemoteCallbacks};
use std::path::Path;
use std::env;

pub fn clone(repo_name: &str, repo_dir: &str) {
    // Prepare callbacks
    let mut callbacks = RemoteCallbacks::new();
    callbacks.credentials(|_url, username_from_url, _allowed_types| {
        Cred::ssh_key(
            username_from_url.unwrap(),
            None,
            Path::new(&format!("{}/.ssh/id_rsa", env::var("HOME").unwrap())),
            None,
        )
    });
    // Prepare fetch options
    let mut fo = git2::FetchOptions::new();
    fo.remote_callbacks(callbacks);

    // Prepare builder
    let mut builder = git2::build::RepoBuilder::new();
    builder.fetch_options(fo);

    // let _ = match Repository::clone(repo_name, repo_dir) {
    let _ = match builder.clone(repo_name, Path::new(repo_dir)) {
        Ok(repo) => repo,
        Err(e) => panic!("Error: {}", e),
    };
}
