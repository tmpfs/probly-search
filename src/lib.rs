mod index;
mod query;
mod utils;

pub use index::*;
pub use query::*;

#[cfg(test)]
pub mod test_util {

    use crate::{
        index::Index,
        query::{score::calculator::ScoreCalculator, QueryResult},
    };
    fn approx_equal(a: f64, b: f64, dp: u8) -> bool {
        let p: f64 = 10f64.powf(-(dp as f64));

        (a - b).abs() < p
    }

    struct Doc {
        id: usize,
        title: String,
    }
    fn tokenizer(s: &str) -> Vec<&str> {
        s.split(' ').collect::<Vec<_>>()
    }
    fn title_extract(d: &Doc) -> Option<&str> {
        Some(d.title.as_str())
    }

    fn filter(s: &str) -> &str {
        s
    }

    pub fn test_score<'arena, M, S: ScoreCalculator<usize, M>>(
        idx: &mut Index<usize>,
        score_calculator: &mut S,
        q: &str,
        expected: Vec<QueryResult<usize>>,
    ) {
        let fields_len = idx.fields.len();
        let mut results = idx.query(
            q,
            score_calculator,
            tokenizer,
            filter,
            &vec![1.; fields_len],
            None,
        );
        results.sort_by(|a, b| {
            let mut sort = b.score.partial_cmp(&a.score).unwrap();
            sort = sort.then_with(|| a.key.partial_cmp(&b.key).unwrap());
            sort
        });

        assert_eq!(expected.len(), results.len());

        for (index, result) in results.iter().enumerate() {
            assert_eq!(expected[index], *result);
            assert_eq!(approx_equal(expected[index].score, result.score, 8), true)
        }
    }

    /***
        Create a index with docucments with title fields, with increasing ids starting from 0
    */

    pub fn build_test_index<'arena>(titles: &[&str]) -> Index<usize> {
        let mut index = Index::<usize>::new(1);
        for (i, title) in titles.iter().enumerate() {
            let doc = Doc {
                id: i,
                title: title.to_string(),
            };
            index.add_document(&[title_extract], tokenizer, filter, doc.id, &doc);
        }
        index
    }
}
