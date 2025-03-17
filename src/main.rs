fn main() {
    println!(
        "Before: {:?}",
        std::env::current_dir().expect("failed to get current_dir")
    );

    std::env::set_current_dir("workdir").expect("failed to set current_dir");

    println!(
        "After: {:?}",
        std::env::current_dir().expect("failed to get current_dir")
    );
}
