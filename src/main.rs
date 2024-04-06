mod generate_password;

fn main() {
    let phrase = generate_password::get_random_phrase();
    for word in phrase {
        println!("{}", word);
    }
}
