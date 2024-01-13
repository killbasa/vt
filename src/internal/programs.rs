use std::env;

pub fn is_program_in_path(program: &str) -> bool {
    match env::var_os("PATH") {
        None => false,
        Some(paths) => env::split_paths(&paths)
            .map(|p| p.join(program))
            .any(|p| p.exists()),
    }
}
