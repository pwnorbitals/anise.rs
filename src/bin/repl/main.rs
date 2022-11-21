
mod cli;
// mod data;


fn main() -> Result<(), impl std::fmt::Debug> {
    cli::exec()
}
