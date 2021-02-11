use nix::unistd::{fork, ForkResult};
use std::process;

// fork() によって、同じプログラムの処理をもう一つのプロセスに分けて処理を続ける.
// cf. https://pubs.opengroup.org/onlinepubs/9699919799/functions/fork.html
fn main() {
    match unsafe { fork() }.expect("fork failed") {
        ForkResult::Parent { child, .. } => {
            println!(
                "I'm parent! My PID = {}, Child Process PID = {}",
                process::id(),
                child
            );
        }
        ForkResult::Child => println!("I'm child! My PID = {}", process::id()),
    }
}
