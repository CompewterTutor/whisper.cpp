use crate::errors::FrontendError;
use std::ffi::OsString;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WhisperCliRequest {
    pub model_path: PathBuf,
    pub audio_path: PathBuf,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WhisperCliOutput {
    pub transcript: String,
    pub raw_stdout: String,
}

pub trait CliRunner {
    fn run(&self, executable: &Path, args: &[OsString]) -> Result<String, FrontendError>;
}

#[derive(Debug, Default, Clone, Copy)]
pub struct ProcessCliRunner;

impl CliRunner for ProcessCliRunner {
    fn run(&self, executable: &Path, args: &[OsString]) -> Result<String, FrontendError> {
        let output = std::process::Command::new(executable)
            .args(args)
            .output()
            .map_err(|source| FrontendError::Io {
                context: "failed to execute whisper-cli",
                source,
            })?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr).to_string();
            return Err(FrontendError::Io {
                context: "whisper-cli exited with non-zero status",
                source: std::io::Error::other(stderr),
            });
        }

        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }
}

pub fn build_whisper_cli_args(request: &WhisperCliRequest) -> Vec<OsString> {
    vec![
        OsString::from("-m"),
        request.model_path.as_os_str().to_os_string(),
        OsString::from("-f"),
        request.audio_path.as_os_str().to_os_string(),
        OsString::from("-otxt"),
        OsString::from("-np"),
    ]
}

pub fn parse_whisper_cli_stdout(stdout: &str) -> String {
    let parsed_lines: Vec<String> = stdout
        .lines()
        .filter_map(|line| {
            let trimmed = line.trim();
            if trimmed.is_empty()
                || trimmed.starts_with("whisper_")
                || trimmed.starts_with("system_info")
            {
                return None;
            }

            let without_time = if let Some(index) = trimmed.find(']') {
                trimmed.get(index + 1..).unwrap_or(trimmed).trim()
            } else {
                trimmed
            };

            if without_time.is_empty() {
                None
            } else {
                Some(without_time.to_owned())
            }
        })
        .collect();

    parsed_lines.join("\n")
}

pub fn run_with_runner(
    executable: &Path,
    request: &WhisperCliRequest,
    runner: &dyn CliRunner,
) -> Result<WhisperCliOutput, FrontendError> {
    let args = build_whisper_cli_args(request);
    let raw_stdout = runner.run(executable, &args)?;
    let transcript = parse_whisper_cli_stdout(&raw_stdout);

    Ok(WhisperCliOutput {
        transcript,
        raw_stdout,
    })
}

#[cfg(test)]
mod tests {
    use super::{
        CliRunner, WhisperCliRequest, build_whisper_cli_args, parse_whisper_cli_stdout,
        run_with_runner,
    };
    use crate::errors::FrontendError;
    use std::ffi::OsString;
    use std::path::{Path, PathBuf};

    struct MockRunner {
        expected_executable: PathBuf,
        expected_arg_count: usize,
        stdout: String,
    }

    impl CliRunner for MockRunner {
        fn run(&self, executable: &Path, args: &[OsString]) -> Result<String, FrontendError> {
            assert_eq!(executable, self.expected_executable.as_path());
            assert_eq!(args.len(), self.expected_arg_count);
            Ok(self.stdout.clone())
        }
    }

    #[test]
    fn build_whisper_cli_args_includes_model_and_audio_flags() {
        let request = WhisperCliRequest {
            model_path: PathBuf::from("models/ggml-base.en.bin"),
            audio_path: PathBuf::from("samples/jfk.wav"),
        };

        let args = build_whisper_cli_args(&request);

        assert_eq!(args[0], OsString::from("-m"));
        assert_eq!(args[2], OsString::from("-f"));
        assert!(args.iter().any(|arg| arg == &OsString::from("-otxt")));
        assert!(args.iter().any(|arg| arg == &OsString::from("-np")));
    }

    #[test]
    fn parse_whisper_cli_stdout_extracts_transcript_lines() {
        let stdout = r#"
whisper_init_from_file: loading model
system_info: n_threads = 8
[00:00.000 --> 00:02.000]  Hello world
[00:02.000 --> 00:04.000]  from whisper cpp
"#;

        let parsed = parse_whisper_cli_stdout(stdout);

        assert_eq!(parsed, "Hello world\nfrom whisper cpp");
    }

    #[test]
    fn run_with_runner_builds_and_parses_output() {
        let executable = PathBuf::from("build/bin/whisper-cli");
        let request = WhisperCliRequest {
            model_path: PathBuf::from("models/ggml-base.en.bin"),
            audio_path: PathBuf::from("samples/jfk.wav"),
        };
        let runner = MockRunner {
            expected_executable: executable.clone(),
            expected_arg_count: 6,
            stdout: "[00:00.000 --> 00:01.000]  test transcript".to_owned(),
        };

        let output = run_with_runner(&executable, &request, &runner)
            .expect("run_with_runner should return parsed output");

        assert_eq!(output.transcript, "test transcript");
        assert!(output.raw_stdout.contains("test transcript"));
    }
}
