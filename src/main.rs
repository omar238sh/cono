mod assets;
mod fonts;
mod render;
fn main() {
    let script = std::env::args().nth(1);
    if let Some(script) = script {
        if let Err(err) = render::lua::run_script(std::path::Path::new(&script)) {
            eprintln!("failed to run lua script: {err}");
            std::process::exit(1);
        }
    }
}
