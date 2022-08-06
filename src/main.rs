mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

pub type PngError = Box<dyn std::error::Error>;
pub type PngResult<T> = std::result::Result<T, PngError>;

fn main() -> PngResult<()> {
    todo!()
}
