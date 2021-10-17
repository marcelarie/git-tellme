use redis::Commands;
use std::process::Command;

// TODO: Make this better
pub fn lift_up_server() -> bool {
    Command::new("redis-server")
        .args(["--daemonize", "yes"])
        .output()
        .expect(
            "Can't start local redis server. Are you sure you have it installed?",
        );
    let check =String::from_utf8( Command::new("ps")
        .args(["-C", "redis-server"])
        .output()
        .expect(
        "Can't start local redis server. Are you sure you have it installed?",
    ).stdout).unwrap();
    match check.contains("redis-server") {
        true => true,
        false => lift_up_server(),
    }
}

fn shut_down_server() {
    Command::new("sh")
        .args(["-c", "kill `pidof redis-server`"])
        .spawn()
        .expect(
        "Can't stop local redis server. Are you sure you have it installed?",
    );
}

pub fn test_server() -> redis::RedisResult<isize> {
    lift_up_server();
    let client = redis::Client::open("redis://127.0.0.1/")?;
    let mut con = client.get_connection()?;
    let _: () = con.set("my_key", 24)?;
    let key = con.get("my_key")?;
    let _: () = con.del("my_key")?;
    shut_down_server();
    println!("Server working");
    Ok(key)
}