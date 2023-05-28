use super::search_index::SearchIndex;
use crate::graph::GraphData;
use crate::graph::GraphImpl;
use crate::graph::GraphIndex;

pub(crate) trait SearchIterator {
    fn expand_edge<Data: GraphData>(index: GraphIndex, graph: &GraphImpl<Data>) -> GraphIndex;
    fn expand_node<Data: GraphData>(index: GraphIndex, graph: &GraphImpl<Data>) -> Vec<GraphIndex>;
    fn new(stack: &mut Vec<SearchIndex>) -> Self;
    fn next(&mut self) -> Option<SearchIndex>;
}
