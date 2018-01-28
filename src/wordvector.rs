use wordvector_base::WordVectorDistance;
use emd::emd;

pub struct EmdDistance;

impl WordVectorDistance for EmdDistance {
    fn calc(&self, doc_bow1: &[f32], doc_bow2: &[f32], distance_matrix: &[f32]) -> f32 {
        emd(doc_bow1, doc_bow2, distance_matrix)
    }
}
