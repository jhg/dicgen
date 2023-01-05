use dicgen::DictionaryGenerator;

use clap::Parser;
use indicatif::ProgressIterator;
use std::path::PathBuf;
use std::io::{Write, BufWriter};

#[derive(Debug, Parser)]
struct Opts {
    #[clap(short, long)]
    alphabet: String,
    #[clap(short, long)]
    init: Option<String>,
    #[clap(short, long)]
    end: String,
    /// If not present, write to stdout.
    #[clap(short, long)]
    file: Option<PathBuf>,
    /// Hide progress bar when writing to file (writing to stdout always hide it).
    #[clap(short, long)]
    without_progress_bar: bool,
}

fn main() {
    let opts = Opts::parse();

    let mut output: BufWriter<Box<dyn Write>> = if let Some(Ok(file)) = opts.file.as_ref().map(std::fs::File::create) {
        BufWriter::new(Box::new(file))
    } else {
        BufWriter::new(Box::new(std::io::stdout().lock()))
    };

    let generator = if let Some(init) = opts.init {
        DictionaryGenerator::new(opts.alphabet, init, opts.end)
    } else {
        DictionaryGenerator::new_from_start(opts.alphabet, opts.end)
    };

    let progress = if opts.file.is_none() || opts.without_progress_bar {
        indicatif::ProgressBar::hidden()
    } else {
        let max = generator.size_hint().1.unwrap_or(generator.size_hint().0);
        indicatif::ProgressBar::new(max as u64)
        .with_style(
            indicatif::ProgressStyle::default_bar()
            .template("{elapsed_precise} {bar:70.cyan/blue} {percent}%")
            .unwrap()
        )
        .with_finish(indicatif::ProgressFinish::AndLeave)
    };

    for value in generator.progress_with(progress) {
        output.write_all(value.as_bytes()).unwrap();
        output.write_all("\n".as_bytes()).unwrap();
    }

    output.flush().unwrap();
}
