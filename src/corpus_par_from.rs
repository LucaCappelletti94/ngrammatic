//! Submodule implementing the `From` trait for the `Corpus` struct.
use rayon::prelude::*;
use sux::prelude::*;
use sux::traits::bit_field_slice::AtomicHelper;

use crate::{bit_field_bipartite_graph::WeightedBitFieldBipartiteGraph, traits::*};

use crate::Corpus;

impl<KS, NG, K> Corpus<KS, NG, K, WeightedBitFieldBipartiteGraph>
where
    NG: Ngram + Send + Sync,
    KS: Keys<NG>,
    for<'a> KS::KeyRef<'a>: AsRef<K>,
    K: Key<NG, NG::G> + ?Sized,
{
    /// Creates a new corpus from a set of keys, in parallel.
    ///
    /// # Arguments
    /// * `keys` - The keys to create the corpus from.
    ///
    /// # Examples
    /// In the following example, we create a corpus from the set of keys
    /// defined by the `ANIMALS` constant array. We provide several synonims
    /// for arrays, such as UniGrams, BiGrams, TriGrams, and so on. This is
    /// solely done for the sake of better readability. The Ngrams are implemented
    /// up to the cardidality of 8, which is the maximum number of `u8`-based grams that can
    /// be stored in a single u64.
    ///
    /// ```rust
    /// use ngrammatic::prelude::*;
    ///
    /// const ANIMALS: [&str; 5] = ["cat", "dog", "bird", "fish", "lion"];
    ///
    /// let bigram_corpus: Corpus<[&str; 5], BiGram<char>> = Corpus::par_from(ANIMALS);
    /// let trigram_corpus: Corpus<[&str; 5], TriGram<char>> = Corpus::par_from(ANIMALS);
    /// let tetragram_corpus: Corpus<[&str; 5], TetraGram<char>> = Corpus::par_from(ANIMALS);
    /// let pentagram_corpus: Corpus<[&str; 5], PentaGram<char>> = Corpus::par_from(ANIMALS);
    /// let hexagram_corpus: Corpus<[&str; 5], HexaGram<char>> = Corpus::par_from(ANIMALS);
    /// let heptagram_corpus: Corpus<[&str; 5], HeptaGram<char>> = Corpus::par_from(ANIMALS);
    /// let octagram_corpus: Corpus<[&str; 5], OctaGram<char>> = Corpus::par_from(ANIMALS);
    /// ```
    ///
    /// You can do the same for vectors of Strings:
    ///
    /// ```rust
    /// use ngrammatic::prelude::*;
    ///
    /// let animals = vec!["cat", "dog", "bird", "fish", "lion"];
    ///
    /// let bigram_corpus: Corpus<Vec<&str>, BiGram<char>> = Corpus::par_from(animals.clone());
    /// let trigram_corpus: Corpus<Vec<&str>, TriGram<char>> = Corpus::par_from(animals.clone());
    /// let tetragram_corpus: Corpus<Vec<&str>, TetraGram<char>> = Corpus::par_from(animals.clone());
    /// let pentagram_corpus: Corpus<Vec<&str>, PentaGram<char>> = Corpus::par_from(animals.clone());
    /// let hexagram_corpus: Corpus<Vec<&str>, HexaGram<char>> = Corpus::par_from(animals.clone());
    /// let heptagram_corpus: Corpus<Vec<&str>, HeptaGram<char>> = Corpus::par_from(animals.clone());
    /// let octagram_corpus: Corpus<Vec<&str>, OctaGram<char>> = Corpus::par_from(animals);
    /// ```
    ///
    /// And references of vectors:
    ///
    /// ```rust
    /// use ngrammatic::prelude::*;
    ///
    /// let animals = vec!["cat", "dog", "bird", "fish", "lion"];
    ///
    /// let bigram_corpus: Corpus<Vec<&str>, BiGram<char>> = Corpus::par_from(animals.clone());
    /// let trigram_corpus: Corpus<Vec<&str>, TriGram<char>> = Corpus::par_from(animals.clone());
    /// let tetragram_corpus: Corpus<Vec<&str>, TetraGram<char>> = Corpus::par_from(animals.clone());
    /// let pentagram_corpus: Corpus<Vec<&str>, PentaGram<char>> = Corpus::par_from(animals.clone());
    /// let hexagram_corpus: Corpus<Vec<&str>, HexaGram<char>> = Corpus::par_from(animals.clone());
    /// let heptagram_corpus: Corpus<Vec<&str>, HeptaGram<char>> = Corpus::par_from(animals.clone());
    /// let octagram_corpus: Corpus<Vec<&str>, OctaGram<char>> = Corpus::par_from(animals.clone());
    /// ```
    ///
    /// And references of arrays:
    ///
    /// ```rust
    /// use ngrammatic::prelude::*;
    ///
    /// let animals = ["cat", "dog", "bird", "fish", "lion"];
    ///
    /// let bigram_corpus: Corpus<[&str; 5], BiGram<char>> = Corpus::par_from(animals);
    /// let trigram_corpus: Corpus<[&str; 5], TriGram<char>> = Corpus::par_from(animals);
    /// let tetragram_corpus: Corpus<[&str; 5], TetraGram<char>> = Corpus::par_from(animals);
    /// let pentagram_corpus: Corpus<[&str; 5], PentaGram<char>> = Corpus::par_from(animals);
    /// let hexagram_corpus: Corpus<[&str; 5], HexaGram<char>> = Corpus::par_from(animals);
    /// let heptagram_corpus: Corpus<[&str; 5], HeptaGram<char>> = Corpus::par_from(animals);
    /// let octagram_corpus: Corpus<[&str; 5], OctaGram<char>> = Corpus::par_from(animals);
    /// ```
    ///
    /// In all of these examples, we have used char-based grams. We can also use u8-based grams:
    ///
    /// ```rust
    /// use ngrammatic::prelude::*;
    ///
    /// let animals = vec!["cat", "dog", "bird", "fish", "lion"];
    ///
    /// let bigram_corpus: Corpus<Vec<&str>, BiGram<u8>> = Corpus::par_from(animals.clone());
    /// let trigram_corpus: Corpus<Vec<&str>, TriGram<u8>> = Corpus::par_from(animals.clone());
    /// let tetragram_corpus: Corpus<Vec<&str>, TetraGram<u8>> = Corpus::par_from(animals.clone());
    /// let pentagram_corpus: Corpus<Vec<&str>, PentaGram<u8>> = Corpus::par_from(animals.clone());
    /// let hexagram_corpus: Corpus<Vec<&str>, HexaGram<u8>> = Corpus::par_from(animals.clone());
    /// let heptagram_corpus: Corpus<Vec<&str>, HeptaGram<u8>> = Corpus::par_from(animals.clone());
    /// let octagram_corpus: Corpus<Vec<&str>, OctaGram<u8>> = Corpus::par_from(animals.clone());
    /// ```
    ///
    /// It is also pretty easy to define normalizations for the keys. For instance, you can
    /// normalize the keys to lowercase:
    ///
    /// ```rust
    /// use ngrammatic::prelude::*;
    ///
    /// let animals = vec!["cat", "dog", "bIrd", "Fish", "Lion"];
    ///
    /// let bigram_corpus: Corpus<Vec<&str>, BiGram<char>, Lowercase<str>> =
    ///     Corpus::par_from(animals.clone());
    /// let trigram_corpus: Corpus<Vec<&str>, TriGram<char>, Lowercase<str>> =
    ///     Corpus::par_from(animals.clone());
    /// let tetragram_corpus: Corpus<Vec<&str>, TetraGram<char>, Lowercase<str>> =
    ///     Corpus::par_from(animals.clone());
    /// let pentagram_corpus: Corpus<Vec<&str>, PentaGram<char>, Lowercase<str>> =
    ///     Corpus::par_from(animals.clone());
    /// let hexagram_corpus: Corpus<Vec<&str>, HexaGram<char>, Lowercase<str>> =
    ///     Corpus::par_from(animals.clone());
    /// let heptagram_corpus: Corpus<Vec<&str>, HeptaGram<char>, Lowercase<str>> =
    ///     Corpus::par_from(animals.clone());
    /// let octagram_corpus: Corpus<Vec<&str>, OctaGram<char>, Lowercase<str>> =
    ///     Corpus::par_from(animals.clone());
    /// ```
    pub fn par_from(keys: KS) -> Self {
        // We start by parsing the keys to extract the ngrams, the cooccurrences, the key offsets,
        // and the maximal cooccurrence.
        let (mut ngrams, cooccurrences_builder, average_key_length, key_offsets, key_to_ngrams) =
            Self::parse_keys(&keys);

        let cooccurrences = cooccurrences_builder.par_build();

        // We sort the ngrams in parallel.
        log::debug!("Sorting ngrams.");
        ngrams.par_sort_unstable();

        // We can now start to compress several of the vectors into BitFieldVecs.
        log::debug!("Compressing key offsets into Elias-Fano.");
        let key_offsets = unsafe { key_offsets.par_into_elias_fano() };

        // We now create the various required bitvectors, knowing all of their characteristics
        // such as the capacity and the largest value to fit in the bitvector, i.e. the number
        // of bits necessary to store the largest value in the vector.

        // We start by creating the ngram_degrees vector. This vector has as length the number of
        // ngrams plus one, and the value at index `i` is the sum of the inbound degrees before
        // index `i`. Since we do not know the actual maximal value of the ngram degrees, we need
        // to use a value that is certainly larger than the maximal value. We can use the number of
        // keys, since at most an ngram will appear in all of the keys. Note that we will not store
        // the ngram comulative degrees in this array, which have their maximal value equal to the
        // number of edges from keys to ngrams, which we already have at this time (i.e. the length
        // of the cooccurrences vector).
        let mut ngram_degrees = BitFieldVec::new(
            (keys.len() + 1).next_power_of_two().ilog2() as usize,
            ngrams.len() + 1,
        );

        // While populating the previous two vectors, we also populate the key_to_ngram_edges.
        // As it stands, this value is populated by the ngrams in the order they appear in the keys. We need
        // to replace these ngrams with their curresponding index, which means that we need to allocate a new
        // vector of the same length as the current key_to_ngram_edges vector, and as maximum value the number
        // of ngrams in the corpus.
        let key_to_ngram_edges = AtomicBitFieldVec::new(
            (ngrams.len() + 1).next_power_of_two().ilog2() as usize,
            key_to_ngrams.len(),
        );

        log::debug!("Building the key to ngram edges.");
        key_to_ngrams
            .into_par_iter()
            .enumerate()
            .for_each(|(edge_id, ngram)| {
                // We find the index of the ngram in the ngrams vector.
                // We can always unwrap since we know that the ngram is in the ngrams vector.
                let ngram_index = unsafe { ngrams.index_of_unchecked(ngram) };
                // We store the index in the key_to_ngram_edges vector.
                unsafe {
                    <AtomicBitFieldVec as AtomicHelper<usize>>::set_unchecked(
                        &key_to_ngram_edges,
                        edge_id,
                        ngram_index,
                        std::sync::atomic::Ordering::SeqCst,
                    )
                };
            });

        // We reconvert the key_to_ngram_edges vector to a non-atomic BitFieldVec.
        let key_to_ngram_edges: BitFieldVec = key_to_ngram_edges.into();

        // We create the ngrams vector. Since we are using a btreeset, we already have the
        // ngrams sorted, so we can simply convert the btreeset into a vector.
        log::debug!(
            "Storing ngrams into {}.",
            std::any::type_name::<NG::SortedStorage>()
        );
        let ngram_builder = <<<NG as Ngram>::SortedStorage as SortedNgramStorage<NG>>::ConcurrentBuilder>::new_storage_builder(ngrams.len(), *ngrams.last().unwrap());

        ngrams
            .into_par_iter()
            .enumerate()
            .for_each(|(index, ngram)| unsafe {
                ngram_builder.set_unchecked(ngram, index);
            });

        let ngrams: NG::SortedStorage = ngram_builder.build();

        log::debug!("Computing ngrams degrees.");

        // We iterate on the key_to_ngrams vector. For each ngram we encounter, we find the index of the ngram
        // in the ngram vector by employing a binary search, since we know that the ngrams are sorted.
        for ngram_id in key_to_ngram_edges.iter() {
            // We increment the inbound degree of the ngram.
            unsafe {
                ngram_degrees
                    .set_unchecked(ngram_id + 1, ngram_degrees.get_unchecked(ngram_id + 1) + 1)
            }
        }

        log::debug!("Computing ngrams degrees comulative sum.");

        // Now that we have fully populated the ngram_degrees vector, we need to compute the comulative
        // sum of the inbound degrees of the ngrams.
        let mut comulative_sum = 0;
        let mut ngram_offsets_builder =
            EliasFanoBuilder::new(ngram_degrees.len(), cooccurrences.num_weights());

        // We iterate on the ngram_degrees vector, and we compute the comulative sum of the inbound degrees.
        for ngram_degree in ngram_degrees.iter() {
            comulative_sum += ngram_degree;
            unsafe { ngram_offsets_builder.push_unchecked(comulative_sum) };
        }

        // We check that the total comulative sum is equal to the number of edges from keys to ngrams.
        debug_assert_eq!(
            comulative_sum,
            cooccurrences.num_weights(),
            "The comulative sum of the ngram degrees should be equal to the number of edges from keys to ngrams."
        );

        // We build the ngram_offsets vector.
        let ngram_offsets = ngram_offsets_builder.build();
        let ngram_offsets = unsafe {
            ngram_offsets.map_high_bits(|high_bits| {
                crate::weights::HighBitsPredEF::new(crate::weights::HighBitsEF::new(high_bits))
            })
        };

        log::debug!("Building edges from gram to key.");
        // Finally, we can allocate and populate the gram_to_key_edges vector. This vector has the same length
        // as the cooccurrences vector.
        let mut gram_to_key_edges = BitFieldVec::new(
            (keys.len() + 1).next_power_of_two().ilog2() as usize,
            cooccurrences.num_weights(),
        );

        // We reset the degrees to zeroes so that we can reuse the ngram_degrees vector.
        ngram_degrees.reset();

        // We iterate on the key_to_ngram_edges while keeping track of the current key, as defined by the key_offsets.
        // For each ngram, by using the ngram_degrees, we can find the position of the key in the gram_to_key_edges vector.

        let mut ngram_iterator = key_to_ngram_edges.iter();

        for (key_id, (key_offset_start, key_offset_end)) in
            key_offsets.iter().zip(key_offsets.iter_from(1)).enumerate()
        {
            // Note that we check for the key offsets to be increasing or equal as
            // it may happen that a key does not contain any ngrams. This could be
            // caused for instance by key containing exclusively invalid characters
            // such as whitespaces or NULL characters.
            debug_assert!(
                key_offset_end >= key_offset_start,
                "The key offsets should be increasing."
            );
            // We iterate on the ngrams of the key.
            for _ in key_offset_start..key_offset_end {
                // We get the next ngram.
                let ngram_id = ngram_iterator.next().unwrap();
                // We get the ngram current degree.
                let ngram_degree: usize = unsafe { ngram_degrees.get_unchecked(ngram_id) };

                // We find the position of the key in the gram_to_key_edges vector.
                let ngram_offset =
                    unsafe { sux::traits::IndexedSeq::get_unchecked(&ngram_offsets, ngram_id) };
                let inbound_edge_id = ngram_offset + ngram_degree;

                // We store the key index in the gram_to_key_edges vector.
                unsafe { gram_to_key_edges.set_unchecked(inbound_edge_id, key_id) };
                //We increment the inbound degree of the key.
                unsafe { ngram_degrees.set_unchecked(ngram_id, ngram_degree + 1) };
            }
        }

        Corpus::new(
            keys,
            ngrams,
            average_key_length,
            WeightedBitFieldBipartiteGraph::new(
                cooccurrences,
                key_offsets,
                ngram_offsets,
                gram_to_key_edges,
                key_to_ngram_edges,
            ),
        )
    }
}
