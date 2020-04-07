//! Test for https://github.com/butte-rs/butte/issues/39

pub mod tests {
    butte_build::include_fbs!("array");
}

use anyhow::{anyhow, Result};
use butte as fb;
use tests::array;

#[test]
fn test_table_array() -> Result<()> {
    // Create a flatbuffers table that holds an array of tables.
    let mut table_entry_vec = vec![];
    let mut enum_entry_vec = vec![];
    let mut int_entry_vec = vec![];
    let mut builder = fb::FlatBufferBuilder::new();
    for i in 0..2 {
        let entry = array::Entry::create(&mut builder, &array::EntryArgs { value: i as u64 });
        table_entry_vec.push(entry);
        enum_entry_vec.push(array::Value::A);
        int_entry_vec.push(i);
    }
    let table_entries = Some(builder.create_vector(&table_entry_vec));
    let enum_entries = Some(builder.create_vector(&enum_entry_vec));
    let int_entries = Some(builder.create_vector(&int_entry_vec));
    let input = array::ArrayTable::create(
        &mut builder,
        &array::ArrayTableArgs {
            table_entries,
            enum_entries,
            int_entries,
        },
    );

    // Serialize.
    builder.finish_minimal(input);
    let raw_bytes = builder.finished_data();

    // Deserialize and verify the result.
    let output = array::ArrayTable::get_root(raw_bytes)?;
    let table_entries = output
        .table_entries()?
        .ok_or(anyhow!("empty table_entries"))?;
    let enum_entries = output
        .table_entries()?
        .ok_or(anyhow!("empty enum_entries"))?;
    let int_entries = output
        .table_entries()?
        .ok_or(anyhow!("empty int_entries"))?;
    assert_eq!(table_entries.len()?, table_entry_vec.len());
    assert_eq!(enum_entries.len()?, enum_entry_vec.len());
    assert_eq!(int_entries.len()?, int_entry_vec.len());

    // Verify the table entry contents.
    for (i, entry) in table_entries.iter().enumerate() {
        let entry = array::Entry::from(entry?);
        assert_eq!(Some(i as u64), entry.value()?);
    }

    Ok(())
}
