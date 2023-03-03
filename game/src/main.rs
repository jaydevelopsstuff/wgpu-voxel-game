use log::info;

fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    info!("Starting game...");

    pollster::block_on(graphics::run());
}
