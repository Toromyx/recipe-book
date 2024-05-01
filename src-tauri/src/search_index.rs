//! TODO Implement this application's search index.
//!
//! TODO start with ingredient search by name.

// TODO maybe https://github.com/quickwit-oss/tantivy as an alternative?

use std::{collections::HashSet, io::Cursor, path::PathBuf};

use milli::{
    documents::{DocumentsBatchBuilder, DocumentsBatchReader},
    heed::{EnvOpenOptions, RwTxn},
    update::{IndexDocuments, IndexDocumentsConfig, IndexerConfig, Settings, UpdateIndexingStep},
    Index,
};

use crate::path::app_data_dir;

/// 10 GB
const MAP_SIZE: usize = 10 * 2_usize.pow(30);

/// TODO doc
///
/// Initialize a named search index.
pub fn search_index_init(
    name: &str,
    primary_key: String,
    searchable_fields: Vec<String>,
    filterable_fields: HashSet<String>,
    sortable_fields: HashSet<String>,
) -> Index {
    let path = search_index_path(name);

    let mut options = EnvOpenOptions::new();
    options.map_size(MAP_SIZE);
    let index = Index::new(options, &path).unwrap();
    let mut rw_txn = index.write_txn().unwrap();

    let config = IndexerConfig::default();
    let mut settings = Settings::new(&mut rw_txn, &index, &config);
    settings.set_primary_key(primary_key);
    settings.set_searchable_fields(searchable_fields);
    settings.set_filterable_fields(filterable_fields);
    settings.set_sortable_fields(sortable_fields);
    settings.set_min_word_len_one_typo(2);
    settings.set_min_word_len_two_typos(4);

    settings.execute(|_| (), || false).unwrap();
    rw_txn.commit().unwrap();
    index
}

pub fn search_index_add(index: &Index, object: &milli::Object) -> Result<(), milli::Error> {
    let mut rw_txn = index.write_txn()?;
    let indexer_config = IndexerConfig::default();
    let index_documents = create_index_documents(&index, &mut rw_txn, &indexer_config)?;
    let mut documents_batch_builder = DocumentsBatchBuilder::new(Vec::new());
    documents_batch_builder.append_json_object(&object)?;
    let documents_batch_reader =
        DocumentsBatchReader::from_reader(Cursor::new(documents_batch_builder.into_inner()?))?;
    let (index_documents, user_error) = index_documents.add_documents(documents_batch_reader)?;
    user_error?;
    index_documents.execute()?;
    rw_txn.commit()?;
    Ok(())
}

pub fn search_index_delete(index: &Index, external_id: String) -> Result<(), milli::Error> {
    let mut rw_txn = index.write_txn()?;
    let indexer_config = IndexerConfig::default();
    let index_documents = create_index_documents(&index, &mut rw_txn, &indexer_config)?;
    let (index_documents, user_error) = index_documents.remove_documents(vec![external_id])?;
    user_error?;
    index_documents.execute()?;
    rw_txn.commit()?;
    Ok(())
}

/// Get the index path.
fn search_index_path(name: &str) -> PathBuf {
    let path = app_data_dir().join("search_index").join(name);
    std::fs::create_dir_all(&path).unwrap();
    path
}

fn create_index_documents<'index, 'rw_txn, 'indexer_config>(
    index: &'index Index,
    rw_txn: &'rw_txn mut RwTxn<'index>,
    indexer_config: &'indexer_config IndexerConfig,
) -> Result<
    IndexDocuments<'rw_txn, 'index, 'indexer_config, fn(UpdateIndexingStep), fn() -> bool>,
    milli::Error,
> {
    let index_documents_progress: fn(UpdateIndexingStep) = |_| {};
    let index_documents_should_abort: fn() -> bool = || false;
    let index_documents = IndexDocuments::new(
        rw_txn,
        index,
        indexer_config,
        IndexDocumentsConfig::default(),
        index_documents_progress,
        index_documents_should_abort,
    )?;
    Ok(index_documents)
}

#[cfg(test)]
mod test {
    use std::io::Cursor;

    use milli::{
        documents::{DocumentsBatchBuilder, DocumentsBatchReader},
        heed::EnvOpenOptions,
        score_details::ScoringStrategy,
        update::{IndexDocuments, IndexDocumentsConfig, IndexerConfig, Settings},
        Index,
    };

    use super::*;
    use crate::tests::run;

    fn documents() -> Vec<Vec<(String, String)>> {
        vec![
            vec![
                ("id".to_string(), "1".to_string()),
                ("name".to_string(), "honey".to_string()),
            ],
            vec![
                ("id".to_string(), "2".to_string()),
                ("name".to_string(), "beef stock".to_string()),
            ],
            vec![
                ("id".to_string(), "3".to_string()),
                ("name".to_string(), "rice".to_string()),
            ],
            vec![
                ("id".to_string(), "4".to_string()),
                ("name".to_string(), "salt".to_string()),
            ],
            vec![
                ("id".to_string(), "5".to_string()),
                ("name".to_string(), "white wine vinegar".to_string()),
            ],
            vec![
                ("id".to_string(), "6".to_string()),
                ("name".to_string(), "oil".to_string()),
            ],
            vec![
                ("id".to_string(), "7".to_string()),
                ("name".to_string(), "sour cream".to_string()),
            ],
            vec![
                ("id".to_string(), "8".to_string()),
                ("name".to_string(), "chicken breast".to_string()),
            ],
            vec![
                ("id".to_string(), "9".to_string()),
                ("name".to_string(), "kebab spice".to_string()),
            ],
            vec![
                ("id".to_string(), "10".to_string()),
                ("name".to_string(), "pepper".to_string()),
            ],
            vec![
                ("id".to_string(), "11".to_string()),
                ("name".to_string(), "lemon juice".to_string()),
            ],
            vec![
                ("id".to_string(), "12".to_string()),
                ("name".to_string(), "sugar".to_string()),
            ],
            vec![
                ("id".to_string(), "13".to_string()),
                ("name".to_string(), "mustard".to_string()),
            ],
            vec![
                ("id".to_string(), "14".to_string()),
                ("name".to_string(), "salad".to_string()),
            ],
            vec![
                ("id".to_string(), "15".to_string()),
                ("name".to_string(), "chives".to_string()),
            ],
        ]
    }

    fn primary_key() -> String {
        "id".to_string()
    }

    fn searchable_fields() -> Vec<String> {
        vec!["name".to_string()]
    }

    fn filterable_fields() -> HashSet<String> {
        HashSet::from([])
    }

    fn sortable_fields() -> HashSet<String> {
        HashSet::from([])
    }

    fn search_index() -> Index {
        let path = search_index_path("test_index");

        let mut options = EnvOpenOptions::new();
        options.map_size(MAP_SIZE);
        Index::new(options, &path).unwrap()
    }

    fn init_search_index() {
        let index = search_index();
        let mut wtxn = index.write_txn().unwrap();

        let config = IndexerConfig::default();
        let mut settings = Settings::new(&mut wtxn, &index, &config);
        settings.set_primary_key(primary_key());
        settings.set_searchable_fields(searchable_fields());
        settings.set_filterable_fields(filterable_fields());
        settings.set_sortable_fields(sortable_fields());
        settings.set_min_word_len_one_typo(2);
        settings.set_min_word_len_two_typos(4);

        settings.execute(|_| (), || false).unwrap();
        wtxn.commit().unwrap();
    }

    fn fill_search_index() {
        let index = search_index();
        let mut wtxn = index.write_txn().unwrap();
        let config = IndexerConfig::default();
        let indexing_config = IndexDocumentsConfig::default();

        let index_documents = IndexDocuments::new(
            &mut wtxn,
            &index,
            &config,
            indexing_config,
            |_| (),
            || false,
        )
        .unwrap();

        let mut documents_batch_builder = DocumentsBatchBuilder::new(Vec::new());
        for document in documents() {
            documents_batch_builder
                .append_json_object(&milli::Object::from_iter(
                    document
                        .into_iter()
                        .map(|(key, value)| (key, serde_json::Value::String(value))),
                ))
                .unwrap();
        }

        let documents_batch_reader = DocumentsBatchReader::from_reader(Cursor::new(
            documents_batch_builder.into_inner().unwrap(),
        ))
        .unwrap();

        let (index_documents, user_error) = index_documents
            .add_documents(documents_batch_reader)
            .unwrap();
        user_error.unwrap();
        index_documents.execute().unwrap();
        wtxn.commit().unwrap();
    }

    fn search_search_index(query: &str) -> milli::SearchResult {
        let index = search_index();
        let ro_txn = index.read_txn().unwrap();
        let mut search = milli::Search::new(&ro_txn, &index);
        search.scoring_strategy(ScoringStrategy::Detailed);
        search.query(query);
        search.execute().unwrap()
    }

    #[test]
    fn test_search_index() {
        run();

        init_search_index();
        fill_search_index();

        let search_result = search_search_index("ustard");

        println!("{:#?}", search_result);
        let index = search_index();
        let ro_txn = index.read_txn().unwrap();
        let external_ids = index
            .external_id_of(&ro_txn, search_result.documents_ids.iter().copied())
            .unwrap()
            .into_iter()
            .filter_map(|s| s.ok())
            .collect::<Vec<_>>();
        println!("external_ids:\n{:#?}", external_ids);

        todo!("make this test actually assert something useful and more maintanable")
    }
}
