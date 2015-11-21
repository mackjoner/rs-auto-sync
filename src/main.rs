extern crate fsevent;

use std::thread;
use std::sync::mpsc::channel;
use std::process::Command;
use std::env;

fn main() {

    /*
    local_path such as: /a/b/c
    remote_path maybe: /x/y/z

    changed_file_path maybe: /a/b/c/d
    relative path = /a/b/c/d - /a/b/c = /d
    so target remote path = /x/y/z + /d = /x/y/z/d
    */

    let args: Vec<_> = env::args().collect();
    if args.len() != 3 {
        println!("rs-auto-sync local_path remote_path");
        return;
    }

    let mut local_path = args[1].clone();
    let mut remote_path = args[2].clone();
    if local_path.starts_with(".") {
        println!("plz rewrite local_path start_with ~ or \\");
        return;
    }
    local_path = local_path.trim_right_matches('/').to_string();
    remote_path = remote_path.trim_right_matches('/').to_string();

    //watch fs event
    let (event_tx, event_rx) = channel();
    let watch_path = local_path.clone();
    thread::spawn(move || {
        let fsevent = fsevent::FsEvent::new(event_tx);
        fsevent.append_path(&watch_path);
        fsevent.observe();
    });

    // fs event handled here
    loop {
        let result = event_rx.recv();
        if !result.is_ok(){
            continue;
        }
        let event = result.unwrap();
        println!(">>> {:?}", event);
        //ignore hide file or dir
        if !event.path.find("/.").is_none() {
            continue;
        }
        let local_relative_path;
        unsafe {
            local_relative_path = event.path.slice_unchecked(
                local_path.len(),
                event.path.len()
            ).to_string();
        }

        let target_remote_path = remote_path.clone() + &local_relative_path;
        let options = vec!["-r", "-v"];
        let output = Command::new("rsync")
            .args(&options)
            .arg(&event.path)
            .arg(&target_remote_path)
            .output()
            .unwrap_or_else(|e| {
                panic!("failed to execute process: {}", e)
            });

        if output.stdout.len() > 0 {
            println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
        }
        if output.stderr.len() > 0 {
            println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
        }
    }
}
