//! Trait defining a key and its hasher.

use crate::traits::ascii_char::ToASCIICharIterator;
use crate::traits::iter_ngrams::IntoNgrams;
use crate::{
    ASCIIChar, ASCIICharIterator, Alphanumeric, BothPadding, CharLike, CharNormalizer, Gram,
    IntoPadder, Lowercase, Ngram, PaddableNgram, SpaceNormalizer, Trim, TrimNull,
};
use fxhash::FxBuildHasher;
use std::collections::HashMap;

/// Trait defining a key.
pub trait Key<NG: Ngram<G = G>, G: Gram>: AsRef<<Self as Key<NG, G>>::Ref> {
    /// The type of the grams iterator.
    type Grams<'a>: Iterator<Item = G>
    where
        Self: 'a;

    /// Default reference type when no more specific type is
    /// specified in the corpus.
    type Ref: ?Sized;

    /// Returns an iterator over the grams of the key.
    ///
    /// # Example
    ///
    /// The following example demonstrates how to get the grams of a key
    /// represented by a string, composed of `u8`:
    /// ```rust
    /// use ngrammatic::prelude::*;
    ///
    /// let key = "abc";
    /// let grams: Vec<u8> = <&str as Key<BiGram<u8>, u8>>::grams(&key).collect();
    /// assert_eq!(grams, vec![b'\0', b'a', b'b', b'c', b'\0',]);
    /// ```
    ///
    /// The following example demonstrates how to get the grams of a key
    /// represented by a string, composed of `char`:
    /// ```rust
    /// use ngrammatic::prelude::*;
    ///
    /// let key = "abc";
    /// let grams: Vec<char> = <&str as Key<BiGram<char>, char>>::grams(&key).collect();
    /// assert_eq!(grams, vec!['\0', 'a', 'b', 'c', '\0']);
    /// ```
    ///
    /// The following example demonstrates how to get the grams of a key
    /// represented by a string, composed of `ASCIIChar`:
    /// ```rust
    /// use ngrammatic::prelude::*;
    ///
    /// let key = "abc";
    /// let grams: Vec<ASCIIChar> = <&str as Key<BiGram<ASCIIChar>, ASCIIChar>>::grams(&key).collect();
    /// assert_eq!(
    ///     grams,
    ///     vec![
    ///         ASCIIChar::from(b'\0'),
    ///         ASCIIChar::from(b'a'),
    ///         ASCIIChar::from(b'b'),
    ///         ASCIIChar::from(b'c'),
    ///         ASCIIChar::from(b'\0')
    ///     ]
    /// );
    /// ```
    fn grams(&self) -> Self::Grams<'_>;

    /// Returns the counts of the ngrams.
    ///
    /// # Example
    ///
    /// The following example demonstrates how to get the counts of the ngrams
    /// of a key represented by a string, composed of `u8`:
    fn counts(&self) -> HashMap<NG, usize, FxBuildHasher> {
        let mut ngram_counts: HashMap<NG, usize, FxBuildHasher> =
            HashMap::with_hasher(FxBuildHasher::default());

        // We populate it with the ngrams of the key.
        for ngram in self.grams().ngrams::<NG>() {
            ngram_counts
                .entry(ngram)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }

        ngram_counts
    }
}

impl<NG> Key<NG, char> for String
where
    NG: Ngram<G = char> + PaddableNgram,
{
    type Grams<'a> =
        BothPadding<NG, SpaceNormalizer<Alphanumeric<TrimNull<Trim<std::str::Chars<'a>>>>>>;

    type Ref = str;

    #[inline(always)]
    fn grams(&self) -> Self::Grams<'_> {
        self.chars()
            .trim()
            .trim_null()
            .alphanumeric()
            .dedup_spaces()
            .both_padding::<NG>()
    }
}

impl<NG> Key<NG, char> for str
where
    NG: Ngram<G = char> + PaddableNgram,
{
    type Grams<'a> = BothPadding<NG, SpaceNormalizer<Alphanumeric<TrimNull<Trim<std::str::Chars<'a>>>>>> where Self: 'a;
    type Ref = str;

    #[inline(always)]
    fn grams(&self) -> Self::Grams<'_> {
        self.chars()
            .trim()
            .trim_null()
            .alphanumeric()
            .dedup_spaces()
            .both_padding::<NG>()
    }
}

impl<NG> Key<NG, u8> for str
where
    NG: Ngram<G = u8> + PaddableNgram,
{
    type Grams<'a> = BothPadding<NG, std::str::Bytes<'a>> where Self: 'a;
    type Ref = str;

    #[inline(always)]
    fn grams(&self) -> Self::Grams<'_> {
        self.bytes().both_padding::<NG>()
    }
}

impl<NG> Key<NG, ASCIIChar> for str
where
    NG: Ngram<G = ASCIIChar> + PaddableNgram,
{
    type Grams<'a> = BothPadding<NG, SpaceNormalizer<Alphanumeric<TrimNull<Trim<ASCIICharIterator<std::str::Chars<'a>>>>>>> where Self: 'a;
    type Ref = str;

    #[inline(always)]
    fn grams(&self) -> Self::Grams<'_> {
        self.chars()
            .ascii()
            .trim()
            .trim_null()
            .alphanumeric()
            .dedup_spaces()
            .both_padding::<NG>()
    }
}

impl<NG> Key<NG, char> for &str
where
    NG: Ngram<G = char> + PaddableNgram,
{
    type Grams<'a> = BothPadding<NG, SpaceNormalizer<Alphanumeric<TrimNull<Trim<std::str::Chars<'a>>>>>> where Self: 'a;
    type Ref = str;

    #[inline(always)]
    fn grams(&self) -> Self::Grams<'_> {
        self.chars()
            .trim()
            .trim_null()
            .alphanumeric()
            .dedup_spaces()
            .both_padding::<NG>()
    }
}

impl<NG> Key<NG, u8> for String
where
    NG: Ngram<G = u8> + PaddableNgram,
{
    type Grams<'a> = BothPadding<NG, std::str::Bytes<'a>>;
    type Ref = str;

    #[inline(always)]
    fn grams(&self) -> Self::Grams<'_> {
        self.bytes().both_padding::<NG>()
    }
}

impl<NG> Key<NG, u8> for &str
where
    NG: Ngram<G = u8> + PaddableNgram,
{
    type Grams<'a> = BothPadding<NG, std::str::Bytes<'a>> where Self: 'a;
    type Ref = str;

    #[inline(always)]
    fn grams(&self) -> Self::Grams<'_> {
        self.bytes().both_padding::<NG>()
    }
}

impl<NG> Key<NG, ASCIIChar> for String
where
    NG: Ngram<G = ASCIIChar> + PaddableNgram,
{
    type Grams<'a> = BothPadding<NG, SpaceNormalizer<Alphanumeric<TrimNull<Trim<ASCIICharIterator<std::str::Chars<'a>>>>>>> where Self: 'a;
    type Ref = str;

    #[inline(always)]
    fn grams(&self) -> Self::Grams<'_> {
        self.chars()
            .ascii()
            .trim()
            .trim_null()
            .alphanumeric()
            .dedup_spaces()
            .both_padding::<NG>()
    }
}

impl<NG> Key<NG, ASCIIChar> for &str
where
    NG: Ngram<G = ASCIIChar> + PaddableNgram,
{
    type Grams<'a> = BothPadding<NG, SpaceNormalizer<Alphanumeric<TrimNull<Trim<ASCIICharIterator<std::str::Chars<'a>>>>>>> where Self: 'a;
    type Ref = str;

    #[inline(always)]
    fn grams(&self) -> Self::Grams<'_> {
        self.chars()
            .ascii()
            .trim()
            .trim_null()
            .alphanumeric()
            .dedup_spaces()
            .both_padding::<NG>()
    }
}

impl<W, NG> Key<NG, NG::G> for Lowercase<W>
where
    NG: Ngram,
    W: Key<NG, NG::G> + ?Sized,
    NG::G: CharLike,
    Self: AsRef<<W as Key<NG, <NG as Ngram>::G>>::Ref>,
{
    type Grams<'a> = Lowercase<W::Grams<'a>> where Self: 'a;
    type Ref = W::Ref;

    #[inline(always)]
    fn grams(&self) -> Self::Grams<'_> {
        self.inner().grams().lower()
    }
}

impl<W, NG> Key<NG, NG::G> for Alphanumeric<W>
where
    NG: Ngram,
    W: Key<NG, NG::G> + ?Sized,
    NG::G: CharLike,
    Self: AsRef<<W as Key<NG, <NG as Ngram>::G>>::Ref>,
{
    type Grams<'a> = Alphanumeric<W::Grams<'a>> where Self: 'a;
    type Ref = W::Ref;

    #[inline(always)]
    fn grams(&self) -> Self::Grams<'_> {
        self.inner().grams().alphanumeric()
    }
}
