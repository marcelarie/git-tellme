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

fn _shut_down_server() {
    Command::new("sh")
        .args(["-c", "kill `pidof redis-server`"])
        .spawn()
        .expect(
        "Can't stop local redis server. Are you sure you have it installed?",
    );
}

pub fn _test_server() -> redis::RedisResult<isize> {
    lift_up_server();
    let client = redis::Client::open("redis://127.0.0.1/")?;
    let mut con = client.get_connection()?;
    let _: () = con.set("my_key", 24)?;
    let key = con.get("my_key")?;
    let _: () = con.del("my_key")?;
    _shut_down_server();
    println!("Server working");
    Ok(key)
}

pub fn save_token(token: &str) -> redis::RedisResult<()> {
    lift_up_server();
    let mut con =
        redis::Client::open("redis://127.0.0.1/")?.get_connection()?;
    let _: () = con.set("TOKEN", token)?;
    Ok(())
}

pub fn get_token() -> redis::RedisResult<String> {
    lift_up_server();
    let mut con =
        redis::Client::open("redis://127.0.0.1/")?.get_connection()?;
    Ok(con.get("TOKEN")?)
}

pub fn _get(key: &str) -> redis::RedisResult<String> {
    lift_up_server();
    let mut con =
        redis::Client::open("redis://127.0.0.1/")?.get_connection()?;
    Ok(con.get(key)?)
}

pub fn _set(key: &str, value: &str) -> redis::RedisResult<String> {
    lift_up_server();
    let mut con =
        redis::Client::open("redis://127.0.0.1/")?.get_connection()?;
    Ok(con.set(key, value)?)
}

pub fn hset(hash: &str, key: &str, value: &str) -> redis::RedisResult<i32> {
    lift_up_server();
    let mut con =
        redis::Client::open("redis://127.0.0.1/")?.get_connection()?;
    Ok(con.hset(hash, key, value)?)
}

pub fn hget(hash: &str, key: &str) -> redis::RedisResult<String> {
    lift_up_server();
    let mut con =
        redis::Client::open("redis://127.0.0.1/")?.get_connection()?;
    Ok(con.hget(hash, key)?)
}

pub fn hkeys(hash: &str) -> redis::RedisResult<Vec<String>> {
    lift_up_server();
    let mut con =
        redis::Client::open("redis://127.0.0.1/")?.get_connection()?;
    Ok(con.hkeys(hash)?)
}

pub fn hdel(hash: &str, field: &str) -> redis::RedisResult<Vec<String>> {
    lift_up_server();
    let mut con =
        redis::Client::open("redis://127.0.0.1/")?.get_connection()?;
    Ok(con.hdel(hash, field)?)
}
