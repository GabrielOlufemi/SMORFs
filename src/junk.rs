/// Lock files: machine-generated, huge, never reviewed by hand.
const JUNK_FILES: &[&str] = &[
    // Rust
    "Cargo.lock",
    // JavaScript / TypeScript
    "package-lock.json",
    "npm-shrinkwrap.json",
    "yarn.lock",
    "pnpm-lock.yaml",
    "bun.lockb",
    // Python
    "poetry.lock",
    "Pipfile.lock",
    "uv.lock",
    "pdm.lock",
    // Go
    "go.sum",
    // Ruby
    "Gemfile.lock",
    // PHP
    "composer.lock",
    // Dart / Flutter
    "pubspec.lock",
    // Swift / iOS
    "Package.resolved",
    "Podfile.lock",
    // Elixir
    "mix.lock",
    // .NET
    "packages.lock.json",
];

/// Folders holding installed dependencies, build output or caches.
const JUNK_DIRS: &[&str] = &[
    // Dependencies
    "node_modules",
    "vendor",
    "bower_components",
    "Pods",
    // Python environments and caches
    ".venv",
    "venv",
    "__pycache__",
    ".pytest_cache",
    ".mypy_cache",
    ".ruff_cache",
    ".tox",
    // Build output
    "target",
    "dist",
    "build",
    ".next",
    ".nuxt",
    "out",
    "bin",
    "obj",
    // Coverage reports
    "coverage",
    "htmlcov",
];

pub fn is_junk(path: &str) -> bool {
    let file_name = path.rsplit('/').next().unwrap_or(path);
    if JUNK_FILES.contains(&file_name) {
        return true;
    }

    path.split('/').any(|folder| JUNK_DIRS.contains(&folder))
}