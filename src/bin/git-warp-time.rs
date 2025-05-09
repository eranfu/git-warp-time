// SPDX-FileCopyrightText: © 2021 Caleb Maclennan <caleb@alerque.com>
// SPDX-License-Identifier: GPL-3.0-only

use camino::Utf8PathBuf;
use clap::CommandFactory;
use git_warp_time::FileSet;
use git_warp_time::cli::Cli;
use git_warp_time::{get_repo, reset_mtimes, resolve_repo_path};
use glob::MatchOptions;
use snafu::prelude::*;

#[derive(Snafu)]
enum Error {
    #[snafu(display("Current working directory is not a valid Git repository.\n{}", source))]
    NoRepository { source: git_warp_time::Error },

    #[snafu(display("Unable to access repository history.\n{}", source))]
    CouldNotAccessRepository { source: git_warp_time::Error },

    #[snafu(display("Unable to change modification time of files.\n{}", source))]
    UnableToResetMTime { source: git_warp_time::Error },

    #[snafu(display("Path '{}' does not exist", path))]
    PathNotFound { path: String },

    #[snafu(display("Invalid path argument"))]
    UnableToFormPath {},
}

// Clap CLI errors are reported using the Debug trait, but Snafu sets up the Display trait.
// So we delegate. c.f. https://github.com/shepmaster/snafu/issues/110
impl std::fmt::Debug for Error {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self, fmt)
    }
}

type Result<T, E = Error> = std::result::Result<T, E>;

fn main() -> Result<()> {
    let version = option_env!("VERGEN_GIT_DESCRIBE").unwrap_or_else(|| env!("CARGO_PKG_VERSION"));
    let app = Cli::command().version(version);
    let matches = app.get_matches();
    let positionals = matches.get_many::<String>("paths");
    let repo = get_repo().context(NoRepositorySnafu)?;
    let mut opts = git_warp_time::Options::new()
        .dirty(matches.get_flag("dirty"))
        .ignored(matches.get_flag("ignored"))
        .ignore_older(matches.get_flag("ignore_older"))
        .verbose(!matches.get_flag("quiet"));
    if matches.contains_id("paths") {
        let mut paths: FileSet = FileSet::new();
        for path_string in positionals.context(UnableToFormPathSnafu)? {
            for path in glob::glob_with(
                path_string,
                MatchOptions {
                    case_sensitive: false,
                    require_literal_separator: true,
                    require_literal_leading_dot: false,
                },
            )
            .expect("Failed to read glob pattern")
            {
                let path = match path {
                    Ok(path) => Utf8PathBuf::from_path_buf(path)
                        .map_err(|_e| PathNotFoundSnafu { path: path_string }.build())?,
                    Err(e) => {
                        eprintln!("{}", e);
                        continue;
                    }
                };
                if !path.exists() {
                    return PathNotFoundSnafu { path }.fail();
                }
                let path = resolve_repo_path(&repo, path).context(CouldNotAccessRepositorySnafu)?;
                paths.insert(path);
            }
        }
        opts = opts.paths(Some(paths));
    }

    reset_mtimes(&repo, opts).context(UnableToResetMTimeSnafu)?;
    Ok(())
}
