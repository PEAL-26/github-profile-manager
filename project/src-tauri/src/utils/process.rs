use crate::error::AppError;
use std::process::{Command, Output};

/// Executa um comando e captura a saída sem exibir janelas de console.
///
/// Em processos GUI no Windows (`windows_subsystem = "windows"`), processos
/// filhos de console (git, ssh, ssh-keygen) abrem uma nova janela de console
/// a cada execução, a menos que sejam criados com a flag CREATE_NO_WINDOW.
/// Este helper aplica essa flag para que tudo rode internamente.
pub fn run_hidden(program: &str, args: &[&str]) -> Result<Output, AppError> {
    let mut cmd = Command::new(program);
    cmd.args(args);

    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        const CREATE_NO_WINDOW: u32 = 0x0800_0000;
        cmd.creation_flags(CREATE_NO_WINDOW);
    }

    cmd.output()
        .map_err(|e| AppError::SystemError(format!("Falha ao executar {}: {}", program, e)))
}
