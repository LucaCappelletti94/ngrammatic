//! Submodule providing a bitfield bipartite graph which provides a structure
//! storing a bipartite graph into two CSR-like structures composed of bitfields.

use std::iter::Chain;
use std::iter::Map;
use std::iter::Zip;

use mem_dbg::{MemDbg, MemSize};

use sux::bits::BitFieldVec;
use sux::dict::elias_fano::EliasFanoIterator;
use sux::prelude::*;
use sux::traits::BitFieldSliceCore;
use webgraph::traits::RandomAccessLabeling;

use crate::weights::HighBitsPredEF;
use crate::weights::Weights;
use crate::WeightedBipartiteGraph;

#[derive(MemSize, MemDbg, Debug, Clone)]
/// A bipartite graph stored in two CSR-like structures composed of bitfields.
pub struct WeightedBitFieldBipartiteGraph {
    /// Vector containing the number of times a given gram appears in a given key.
    /// This is a descriptor of an edge from a Key to a Gram.
    pub(crate) srcs_to_dsts_weights: Weights,
    /// Vector containing the comulative outbound degree from a given key to grams.
    /// This is a vector with the same length as the keys vector PLUS ONE, and the value at
    /// index `i` is the sum of the oubound degrees before index `i`. The last element of this
    /// vector is the total number of edges in the bipartite graph from keys to grams.
    /// We use this vector alongside the `cooccurrences` vector to find the weighted edges
    /// of a given key. The destinations, i.e. the grams, are found in the `grams` vector.
    srcs_offsets: crate::weights::PredEF,
    /// Vector contain the comulative inbound degree from a given gram to keys.
    /// This is a vector with the same length as the grams vector PLUS ONE, and the value at
    /// index `i` is the sum of the inbound degrees before index `i`. The last element of this
    /// vector is the total number of edges in the bipartite graph from grams to keys.
    /// These edges are NOT weighted, as the weights are stored in the `cooccurrences` vector and
    /// solely refer to the edges from keys to grams.
    dsts_offsets: crate::weights::PredEF,
    /// Vector containing the destinations of the edges from keys to grams.
    srcs_to_dsts: BitFieldVec,
    /// Vector containing the sources of the edges from grams to keys.
    dsts_to_srcs: BitFieldVec,
}

impl WeightedBitFieldBipartiteGraph {
    /// Creates a new `WeightedBitFieldBipartiteGraph`.
    ///
    /// # Arguments
    /// * `srcs_to_dsts_weights` - The weights of the edges from keys to grams.
    /// * `srcs_offsets` - The comulative outbound degree from a given key to grams.
    /// * `dsts_offsets` - The comulative inbound degree from a given gram to keys.
    /// * `srcs_to_dsts` - The destinations of the edges from keys to grams.
    /// * `dsts_to_srcs` - The sources of the edges from grams to keys.
    pub fn new(
        srcs_to_dsts_weights: Weights,
        srcs_offsets: crate::weights::PredEF,
        dsts_offsets: crate::weights::PredEF,
        srcs_to_dsts: BitFieldVec,
        dsts_to_srcs: BitFieldVec,
    ) -> Self {
        assert_eq!(srcs_to_dsts.len(), srcs_to_dsts_weights.num_weights());
        assert_eq!(srcs_to_dsts.len(), dsts_to_srcs.len());

        WeightedBitFieldBipartiteGraph {
            srcs_to_dsts_weights,
            srcs_offsets,
            dsts_offsets,
            srcs_to_dsts,
            dsts_to_srcs,
        }
    }

    /// Returns the comulative outbound degree from a source id.
    ///
    /// # Arguments
    /// * `src_id` - The source id.
    #[inline(always)]
    pub fn src_comulative_outbound_degree(&self, src_id: usize) -> usize {
        self.srcs_offsets.get(src_id)
    }

    /// Returns the comulative inbound degree from a destination id.
    ///
    /// # Arguments
    /// * `dst_id` - The destination id.
    #[inline(always)]
    pub fn dst_comulative_inbound_degree(&self, dst_id: usize) -> usize {
        self.dsts_offsets.get(dst_id)
    }

    /// Returns the src_id from a given edge_id from src to dst.
    ///
    /// # Arguments
    /// * `edge_id` - The edge id.
    ///
    /// # Implementative details
    /// Since the source comulative outbound degree is stored in a bitfield, we can
    /// use the `rank` method to find the source id of a given edge id.
    #[inline(always)]
    pub fn src_id_from_edge_id(&self, edge_id: usize) -> usize {
        self.srcs_offsets.pred(&edge_id).unwrap().0
    }

    /// Returns the dst_id from a given edge_id from src to dst.
    ///
    /// # Arguments
    /// * `edge_id` - The edge id.
    ///
    /// # Implementative details
    /// Since the destination comulative inbound degree is stored in a bitfield, we can
    /// use the `rank` method to find the destination id of a given edge id.
    #[inline(always)]
    pub fn dst_id_from_edge_id(&self, edge_id: usize) -> usize {
        self.dsts_offsets.pred(&edge_id).unwrap().0
    }
}

impl WeightedBipartiteGraph for WeightedBitFieldBipartiteGraph {
    #[inline(always)]
    fn number_of_source_nodes(&self) -> usize {
        self.srcs_offsets.len() - 1
    }

    #[inline(always)]
    fn number_of_destination_nodes(&self) -> usize {
        self.dsts_offsets.len() - 1
    }

    #[inline(always)]
    fn number_of_edges(&self) -> usize {
        self.srcs_to_dsts_weights.num_weights()
    }

    #[inline(always)]
    fn src_degree(&self, src_id: usize) -> usize {
        let start = self.srcs_offsets.get(src_id);
        let end = self.srcs_offsets.get(src_id + 1);
        end - start
    }

    #[inline(always)]
    fn dst_degree(&self, dst_id: usize) -> usize {
        let start = self.dsts_offsets.get(dst_id);
        let end = self.dsts_offsets.get(dst_id + 1);
        end - start
    }

    type Srcs<'a> = std::iter::Take<BitFieldVecIterator<'a, usize, Vec<usize>>>;

    #[inline(always)]
    fn srcs_from_dst(&self, dst_id: usize) -> Self::Srcs<'_> {
        let start = self.dsts_offsets.get(dst_id);
        let end = self.dsts_offsets.get(dst_id + 1);
        self.srcs_to_dsts.iter_from(start).take(end - start)
    }

    type Dsts<'a> = std::iter::Take<BitFieldVecIterator<'a, usize, Vec<usize>>>;

    #[inline(always)]
    fn dsts_from_src(&self, src_id: usize) -> Self::Dsts<'_> {
        let start = self.srcs_offsets.get(src_id);
        let end = self.srcs_offsets.get(src_id + 1);
        self.dsts_to_srcs.iter_from(start).take(end - start)
    }

    type WeightsSrc<'a> = crate::weights::Succ<
        <crate::weights::CursorReaderFactory as crate::weights::ReaderFactory>::Reader<'a>,
    >;

    #[inline(always)]
    fn weights_from_src(&self, src_id: usize) -> Self::WeightsSrc<'_> {
        self.srcs_to_dsts_weights.labels(src_id)
    }

    type Weights<'a> = crate::weights::WeightsIter<
        <crate::weights::CursorReaderFactory as crate::weights::ReaderFactory>::Reader<'a>,
    >;

    #[inline(always)]
    fn weights(&self) -> Self::Weights<'_> {
        self.srcs_to_dsts_weights.weights()
    }

    type Degrees<'a> = Chain<
        Map<
            Zip<
                EliasFanoIterator<'a, HighBitsPredEF, BitFieldVec<usize, Box<[usize]>>>,
                EliasFanoIterator<'a, HighBitsPredEF, BitFieldVec<usize, Box<[usize]>>>,
            >,
            fn((usize, usize)) -> usize,
        >,
        Map<
            Zip<
                EliasFanoIterator<'a, HighBitsPredEF, BitFieldVec<usize, Box<[usize]>>>,
                EliasFanoIterator<'a, HighBitsPredEF, BitFieldVec<usize, Box<[usize]>>>,
            >,
            fn((usize, usize)) -> usize,
        >,
    >;

    #[inline(always)]
    fn degrees(&self) -> Self::Degrees<'_> {
        fn delta((a, b): (usize, usize)) -> usize {
            b - a
        }

        self.srcs_offsets
            .iter()
            .zip(self.srcs_offsets.iter_from(1))
            .map(delta as fn((usize, usize)) -> usize)
            .chain(
                self.dsts_offsets
                    .iter()
                    .zip(self.dsts_offsets.iter_from(1))
                    .map(delta as fn((usize, usize)) -> usize),
            )
    }
}
