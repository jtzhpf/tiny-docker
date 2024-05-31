use std::env;
use std::path::Path;
use std::process::{Command, exit};
use nix::sys::wait::waitpid;
use nix::unistd::{chroot, chdir, fork, sethostname, ForkResult};
use nix::mount::{mount, MsFlags};
use nix::sched::{unshare, CloneFlags};

static CHILD_HOSTNAME: &str = "TinyDocker";

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Too few arguments");
        exit(1);
    }

    if args[1] == "run" {
        run(&args[2..]);
    }
}

fn run(args: &[String]) {
    println!("Parent is running {} pid {}", cmd(&args), std::process::id());

    match unshare(CloneFlags::CLONE_NEWPID) {
        Ok(()) => {}
        Err(e) => {
            eprintln!("Failed to unshare PID namespace: {}", e);
            exit(1);
        }
    }

    match fork() {
        Ok(ForkResult::Child) => {
            run_child(&args);
        }
        Ok(ForkResult::Parent { child }) => {
            let res = waitpid(child, None);
            if res.is_err() {
                eprintln!("Failed to wait for child");
            }
        }
        Err(e) => {
            eprintln!("Failed to fork child: {}", e);
            return;
        }
    }
}

fn cmd(args: &[String]) -> String {
    args.join(" ")
}

fn run_child(args: &[String]) {
    println!("Child is running {} pid {}", cmd(&args), std::process::id());

    let flags = CloneFlags::CLONE_NEWUTS | CloneFlags::CLONE_NEWNS;

    if let Err(e) = unshare(flags) {
        eprintln!("Failed to unshare in child: {}", e);
        exit(1);
    }

    if let Err(e) = mount(None::<&Path>, "/", Some("proc"), MsFlags::MS_SLAVE | MsFlags::MS_REC, None::<&str>) {
        eprintln!("Failed to mount /: {}", e);
        exit(1);
    }

    if let Err(e) = chroot("../docker-fs") {
        eprintln!("Failed to chroot: {}", e);
        exit(1);
    }

    if let Err(e) = chdir("/") {
        eprintln!("Failed to chdir to /: {}", e);
        exit(1);
    }

    if let Err(e) = mount(Some("proc"), "proc", Some("proc"), MsFlags::empty(), None::<&str>){
        eprintln!("Failed to mount /proc: {}", e);
        exit(1);
    }

    if let Err(e) =sethostname(CHILD_HOSTNAME){
        eprintln!("Failed to change hostname: {}", e);
        exit(1);
    }

    if let Err(e) = Command::new(&args[0]).args(&args[1..]).status() {
        eprintln!("Faileded to exec: {}", e);
    }
}
