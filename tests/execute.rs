use std::path::{Path, PathBuf};

use pretty_assertions::assert_eq;
use sauce::shell::Zsh;
use sauce::{output::Output, Context};

fn mkpath(path: &str) -> PathBuf {
    Path::new(path).canonicalize().unwrap().to_path_buf()
}

#[test]
fn it_works_when_no_saucefile_exists() {
    let mut context = Context::default();
    context.sauce_path = Path::new("does_not_exist.toml").to_path_buf();
    let mut output = Output::default();
    let shell_kind = Zsh {};
    context.execute(&shell_kind, &mut output, false);
    assert_eq!(output.result(), "\n\n\n");
    assert_eq!(
        output.message(),
        format!("Sourced {}\n", context.sauce_path.to_string_lossy())
    );
}

#[test]
fn it_runs() {
    let mut context = Context::default();
    context.sauce_path = mkpath("./tests/execute_it_runs.toml");
    let mut output = Output::default();
    let shell_kind = Zsh {};
    context.execute(&shell_kind, &mut output, false);
    assert_eq!(
        output.result(),
        r#"export TEST=example;

alias foo=git;

function meow {
  echo "$@"
};

"#
    );
    assert_eq!(
        output.message(),
        format!("Sourced {}\n", context.sauce_path.to_string_lossy())
    );
}

#[test]
fn it_no_ops_with_autoload_flag_when_autoload_is_disabled() {
    let mut context = Context::default();
    context.sauce_path = mkpath("./tests/execute_it_runs.toml");
    let mut output = Output::default();
    let shell_kind = Zsh {};
    context.execute(&shell_kind, &mut output, true);
    assert_eq!(output.result(), "\n");
    assert_eq!(output.message(), "\n");
}

#[test]
fn it_loads_with_autoload_flag_when_autoload_is_enabled() {
    let mut context = Context::default();
    context.sauce_path = mkpath("./tests/execute_it_runs.toml");
    context.settings.autoload = Some(true);
    let mut output = Output::default();
    let shell_kind = Zsh {};
    context.execute(&shell_kind, &mut output, true);
    assert_eq!(
        output.message(),
        format!("Sourced {}\n", context.sauce_path.to_string_lossy())
    );
}
