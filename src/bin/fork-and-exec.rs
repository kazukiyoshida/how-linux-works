use nix::unistd::{execv, fork, ForkResult};
use std::env;
use std::ffi::CString;
use std::process;

// fork() した後に execv() によって自分のプロセスを新しいプログラムで上書きする.
// 今回は子プロセスを "/bin/echo" で上書き.
fn main() {
    match unsafe { fork() }.expect("fork failed") {
        ForkResult::Parent { child, .. } => {
            println!(
                "I'm parent! My PID = {}, Child Process PID = {}",
                process::id(),
                child
            );
        }
        ForkResult::Child => {
            println!("I'm child! My PID = {}", process::id());

            let args: Vec<String> = env::args().collect();
            let cmd = CString::new("/bin/echo".to_string()).unwrap();
            let msg = CString::new(args[1].to_string()).unwrap();

            execv(&cmd, &[cmd.clone(), msg]).unwrap();
        }
    }
}
