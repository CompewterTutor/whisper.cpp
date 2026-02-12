use crate::errors::FrontendError;
use std::ffi::OsString;
use std::path::{Path, PathBuf};
use std::process::Stdio;
use std::thread;
use std::time::{Duration, Instant};

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct CliRunOptions {
    pub timeout_ms: Option<u64>,
    pub cancel_requested: bool,
}

pub trait CliRunner {
    fn run(
        &self,
        executable: &Path,
        args: &[OsString],
        options: CliRunOptions,
    ) -> Result<String, FrontendError>;
}

#[derive(Debug, Default, Clone, Copy)]
pub struct ProcessCliRunner;

impl CliRunner for ProcessCliRunner {
    fn run(
        &self,
        executable: &Path,
        args: &[OsString],
        options: CliRunOptions,
    ) -> Result<String, FrontendError> {
        if options.cancel_requested {
            return Err(FrontendError::ExecutionCancelled);
        }

        match options.timeout_ms {
            Some(timeout_ms) => run_process_with_timeout(executable, args, timeout_ms),
            None => run_process(executable, args),
        }
    }
}

fn run_process(executable: &Path, args: &[OsString]) -> Result<String, FrontendError> {
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

fn run_process_with_timeout(
    executable: &Path,
    args: &[OsString],
    timeout_ms: u64,
) -> Result<String, FrontendError> {
    let mut child = std::process::Command::new(executable)
        .args(args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|source| FrontendError::Io {
            context: "failed to spawn whisper-cli",
            source,
        })?;

    let started = Instant::now();
    let timeout = Duration::from_millis(timeout_ms);

    loop {
        if started.elapsed() >= timeout {
            let _ = child.kill();
            let _ = child.wait();
            return Err(FrontendError::ExecutionTimeout { timeout_ms });
        }

        match child.try_wait().map_err(|source| FrontendError::Io {
            context: "failed to poll whisper-cli process",
            source,
        })? {
            Some(_status) => {
                let output = child
                    .wait_with_output()
                    .map_err(|source| FrontendError::Io {
                        context: "failed to collect whisper-cli output",
                        source,
                    })?;

                if !output.status.success() {
                    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
                    return Err(FrontendError::Io {
                        context: "whisper-cli exited with non-zero status",
                        source: std::io::Error::other(stderr),
                    });
                }

                return Ok(String::from_utf8_lossy(&output.stdout).to_string());
            }
            None => thread::sleep(Duration::from_millis(10)),
        }
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
    options: CliRunOptions,
) -> Result<WhisperCliOutput, FrontendError> {
    let args = build_whisper_cli_args(request);
    let raw_stdout = runner.run(executable, &args, options)?;
    let transcript = parse_whisper_cli_stdout(&raw_stdout);

    Ok(WhisperCliOutput {
        transcript,
        raw_stdout,
    })
}

#[cfg(test)]
mod tests {
    use super::{
        CliRunOptions, CliRunner, WhisperCliRequest, build_whisper_cli_args,
        parse_whisper_cli_stdout, run_with_runner,
    };
    use crate::errors::FrontendError;
    use std::ffi::OsString;
    use std::path::{Path, PathBuf};

    struct MockRunner {
        expected_executable: PathBuf,
        expected_arg_count: usize,
        expected_options: CliRunOptions,
        stdout: String,
    }

    impl CliRunner for MockRunner {
        fn run(
            &self,
            executable: &Path,
            args: &[OsString],
            options: CliRunOptions,
        ) -> Result<String, FrontendError> {
            assert_eq!(executable, self.expected_executable.as_path());
            assert_eq!(args.len(), self.expected_arg_count);
            assert_eq!(options, self.expected_options);
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
            expected_options: CliRunOptions {
                timeout_ms: Some(2500),
                cancel_requested: false,
            },
            stdout: "[00:00.000 --> 00:01.000]  test transcript".to_owned(),
        };

        let output = run_with_runner(
            &executable,
            &request,
            &runner,
            CliRunOptions {
                timeout_ms: Some(2500),
                cancel_requested: false,
            },
        )
        .expect("run_with_runner should return parsed output");

        assert_eq!(output.transcript, "test transcript");
        assert!(output.raw_stdout.contains("test transcript"));
    }

    #[test]
    fn process_runner_returns_cancelled_when_requested() {
        use super::ProcessCliRunner;

        let runner = ProcessCliRunner;
        let error = runner
            .run(
                Path::new("non-existent-binary"),
                &[],
                CliRunOptions {
                    timeout_ms: None,
                    cancel_requested: true,
                },
            )
            .expect_err("cancelled run should fail early");

        assert!(matches!(error, FrontendError::ExecutionCancelled));
    }
}
