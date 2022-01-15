extern crate anyhow;

use rust_bert::pipelines::translation::{Language, TranslationConfig, TranslationModel};
use tch::Device;

fn main() -> anyhow::Result<()> {
    let translation_config =
        TranslationConfig::new(Language::EnglishToGerman, Device::cuda_if_available());
    let model = TranslationModel::new(translation_config)?;

    let input_context_1 = "The quick brown fox jumps over the lazy dog";
    let input_context_2 = "The dog did not wake up";

    let output = model.translate(&[input_context_1, input_context_2]);

    for sentence in output {
        println!("{}", sentence);
    }
    Ok(())
}