//! # Benchmarks
//!
//! This crate contains memory benchmarks for the `ngrammatic` crate.
//! For the time-related benchmarks, please refer to the benches directory.
//!
//! The memory benchmarks compare different support data-structures that can be used to store the n-grams.
//! As corpus we use the `./taxons.csv.gz` file, which contains a single column with the scientific names
//! of the taxons as provided by NCBI Taxonomy.
use core::fmt::Debug;
use indicatif::ProgressIterator;
use mem_dbg::*;
use ngrammatic::prelude::*;
use rayon::prelude::*;
use std::io::Write;

/// Returns an iterator over the taxons in the corpus.
fn iter_taxons() -> impl Iterator<Item = String> {
    use flate2::read::GzDecoder;
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    let file = File::open("./taxons.csv.gz").unwrap();
    let reader = BufReader::new(GzDecoder::new(file));
    reader.lines().map(|line| line.unwrap())
}

/// Returns bigram corpus.
fn load_corpus_new<NG>()
where
    NG: Ngram<G = ASCIIChar>,
{
    let number_of_taxons = 2_571_000;
    let start_time = std::time::Instant::now();
    let taxons: Vec<String> = iter_taxons().collect();
    let corpus: Corpus<Vec<String>, NG, Lowercase<str>> = Corpus::from(taxons);

    let end_time = std::time::Instant::now();
    let duration: usize = (end_time - start_time).as_millis() as usize;

    // While this is a simple info message, we use the error flag so that the log will
    // not get polluted by the log messages of the other dependencies which can, at times
    // be quite significant.
    log::error!(
        "NEW - Arity: {}, Time (ms): {}, memory (B): {}",
        NG::ARITY,
        duration.underscored(),
        corpus.mem_size(SizeFlags::default()).underscored()
    );
}

/// Returns bigram corpus.
fn load_corpus_par_new<NG>()
where
    NG: Ngram<G = ASCIIChar>,
{
    let start_time = std::time::Instant::now();
    let taxons: Vec<String> = iter_taxons().collect();
    let corpus: Corpus<Vec<String>, NG, Lowercase<str>> = Corpus::par_from(taxons);

    let end_time = std::time::Instant::now();
    let duration: usize = (end_time - start_time).as_millis() as usize;

    // While this is a simple info message, we use the error flag so that the log will
    // not get polluted by the log messages of the other dependencies which can, at times
    // be quite significant.
    log::error!(
        "NEWPAR - Arity: {}, Time (ms): {}, memory (B): {}, memory graph (B): {}",
        NG::ARITY,
        duration.underscored(),
        corpus.mem_size(SizeFlags::default()).underscored(),
        corpus.graph().mem_size(SizeFlags::default()).underscored()
    );
}

fn load_corpus_old(arity: usize) -> ngrammatic_old::Corpus {
    let start_time = std::time::Instant::now();
    let mut corpus: ngrammatic_old::Corpus = ngrammatic_old::CorpusBuilder::new()
        .arity(arity)
        .pad_full(ngrammatic_old::Pad::Auto)
        .finish();

    for line in iter_taxons() {
        corpus.add_text(&line);
    }

    let end_time = std::time::Instant::now();
    let duration: usize = (end_time - start_time).as_millis() as usize;

    // While this is a simple info message, we use the error flag so that the log will
    // not get polluted by the log messages of the other dependencies which can, at times
    // be quite significant.
    log::error!(
        "OLD - Arity: {}, Time (ms): {}, memory (B): {}",
        arity,
        duration.underscored(),
        corpus.mem_size(SizeFlags::default()).underscored()
    );

    corpus
}

/// Returns bigram corpus.
fn load_corpus_webgraph<NG>()
where
    NG: Ngram<G = ASCIIChar> + Debug,
{
    // let number_of_taxons = 2_571_000;
    let number_of_taxons = 2_571_000;

    let loading_bar = indicatif::ProgressBar::new(number_of_taxons as u64);

    let progress_style = indicatif::ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})")
        .unwrap()
        .progress_chars("#>-");

    loading_bar.set_style(progress_style);

    let start_time = std::time::Instant::now();
    let taxons: Vec<String> = iter_taxons()
        .take(number_of_taxons)
        .progress_with(loading_bar)
        .collect();
    let corpus: Corpus<Vec<String>, NG, Lowercase<str>> = Corpus::par_from(taxons);

    let corpus_webgraph: Corpus<Vec<String>, NG, Lowercase<str>, BiWebgraph> =
        Corpus::try_from(corpus).unwrap();

    let end_time = std::time::Instant::now();
    let duration: usize = (end_time - start_time).as_millis() as usize;

    // While this is a simple info message, we use the error flag so that the log will
    // not get polluted by the log messages of the other dependencies which can, at times
    // be quite significant.
    log::error!(
        "WEBGRAPH - Arity: {}, Time (ms): {}, memory (B): {}, memory graph (B): {}",
        NG::ARITY,
        duration.underscored(),
        corpus_webgraph.mem_size(SizeFlags::default() | SizeFlags::FOLLOW_REFS).underscored(),
        corpus_webgraph.graph().mem_size(SizeFlags::default() | SizeFlags::FOLLOW_REFS).underscored()
    );
}

/// Returns bigram corpus.
fn bigram_corpus() {
    load_corpus_new::<BiGram<ASCIIChar>>()
}

/// Returns trigram corpus.
fn trigram_corpus() {
    load_corpus_new::<TriGram<ASCIIChar>>()
}

fn experiment<NG>()
where
    NG: Ngram<G = ASCIIChar>,
{
    // load_corpus_new::<NG>();
    load_corpus_par_new::<NG>();
    load_corpus_webgraph::<NG>();
    load_corpus_old(NG::ARITY);
}

fn main() {
    env_logger::builder().try_init().unwrap();
    experiment::<MonoGram<ASCIIChar>>();
    experiment::<BiGram<ASCIIChar>>();
    experiment::<TriGram<ASCIIChar>>();
    experiment::<TetraGram<ASCIIChar>>();
    experiment::<PentaGram<ASCIIChar>>();
    experiment::<HexaGram<ASCIIChar>>();
    experiment::<HeptaGram<ASCIIChar>>();
    experiment::<OctaGram<ASCIIChar>>();
}
