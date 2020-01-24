use std::fs::{create_dir, remove_dir_all};
use std::path::Path;

use tantivy::schema::Field;
use tantivy::schema::*;
use tantivy::{doc, Index};

fn create_schema() -> Schema {
    let mut schema_builder = Schema::builder();
    schema_builder.add_text_field("title", TEXT | STORED);
    schema_builder.add_u64_field("id", FAST | STORED);
    schema_builder.build()
}

fn create_index() -> tantivy::Result<Index> {
    let schema = create_schema();

    let check_path = Path::new("/tmp/tantivy/idxbs");
    let dir_exists = check_path.exists();
    if dir_exists {
        remove_dir_all(check_path).expect("dir does not exist");
    }

    let index_path = Path::new("/tmp/tantivy/idxbs");
    create_dir(index_path).expect("dir already exists");

    let index = Index::create_in_dir(&index_path, schema.clone())?;

    let mut index_writer = index.writer_with_num_threads(1, 3_000_000)?;
    let title = index.schema().get_field("title").unwrap();
    let id: Field = index.schema().get_field("id").unwrap();
    index_writer.add_document(doc!(title => "The Diary of Muadib", id => 1u64));
    index_writer.add_document(doc!(title => "A Dairy Cow", id => 10u64));
    index_writer.commit()?;
    Ok(index)
}

fn main() -> tantivy::Result<()> {
    let _index = create_index().unwrap();
    Ok(())
}
