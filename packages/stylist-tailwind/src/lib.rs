pub use stylist_tailwind_macros::tw;

mod variables;

pub fn initialize() -> Result<(), stylist::Error> {
    variables::initialize()
}
