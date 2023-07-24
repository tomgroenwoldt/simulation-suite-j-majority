use anyhow::Result;
use console::Emoji;
use indicatif::{ProgressBar, ProgressStyle};

pub static CHECKMARK: Emoji<'_, '_> = Emoji("✅", "");
pub static FACTORY: Emoji<'_, '_> = Emoji("🏭", "");
pub static FLOPPY_DISK: Emoji<'_, '_> = Emoji("💾", "");
pub static FOLDER: Emoji<'_, '_> = Emoji("📂", "");
pub static GRAPH: Emoji<'_, '_> = Emoji("📈", "");
pub static ROCKET: Emoji<'_, '_> = Emoji("🚀", "");
pub static TOOLS: Emoji<'_, '_> = Emoji("🛠️ ", "");

pub fn create_progress_bar(max_value: u64) -> Result<ProgressBar> {
    let progress_bar = ProgressBar::new(max_value);
    progress_bar.set_style(ProgressStyle::with_template(
        "[{elapsed_precise}] {bar:.cyan/blue} {pos}/{len} {msg}",
    )?);

    Ok(progress_bar)
}
