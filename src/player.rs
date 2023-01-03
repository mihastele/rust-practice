pub fn play_movie(name: &str) {
    // pub exposes function to other files
    println!("Playing Movie {}", name);
}
fn play_audio(name: &str) {
    println!("Playing Audio {}", name);
}
