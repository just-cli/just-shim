fn main() {
    use std::fs::File;
    use std::io::Read;

    let args: Vec<String> = std::env::args().collect();
    let exe_name = args.first().unwrap();
    let exe_path = File::open(exe_name)
        .and_then(|mut file| {
            let mut buf = String::new();
            file.read_to_string(&mut buf)
                .and_then(|_| Ok(buf.trim().to_owned()))
        })
        .unwrap_or_else(|e| panic!("No path for {:?}: {:?}", exe_name, e));

    duct::cmd(&exe_path, &args[1..]).run().unwrap_or_else(|e| {
        panic!(
            "Could not execute {:?} with path {:?}: {:?}",
            exe_name, exe_path, e
        )
    });
}
