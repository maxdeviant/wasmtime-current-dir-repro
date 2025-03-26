fn main() {
    let pwd = std::env::var("PWD").expect("failed to get PWD");
    println!("PWD: {pwd:?}");

    println!(
        "Before: {:?}",
        std::env::current_dir().expect("failed to get current_dir")
    );

    std::env::set_current_dir(pwd).expect("failed to set current_dir");

    println!(
        "After: {:?}",
        std::env::current_dir().expect("failed to get current_dir")
    );

    println!("OS: {}", std::env::consts::OS);
}
