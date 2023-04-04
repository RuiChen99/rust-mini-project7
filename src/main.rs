use rand::Rng;

fn main() {
    let password = generate_password(12);
    println!("I recommand your password could be: {}", password);
}

fn generate_password(length: usize) -> String {
    let chars = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!@#$%^&*()_+-=[]{}|;:,.<>?";
    let mut rng = rand::thread_rng();
    let password: String = (0..length)
        .map(|_| rng.gen_range(0..chars.len()))
        .map(|i| chars.chars().nth(i).unwrap())
        .collect();
    password
}