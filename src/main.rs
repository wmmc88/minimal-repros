use std::path::Path;

use ignore::WalkBuilder;

fn main(){
    let initial_folder = dbg!(Path::new(file!()).ancestors().nth(2).expect("2nd ancestor should exist").join("example-folder").canonicalize().expect("example-folder path should exist"));

    WalkBuilder::new(&initial_folder)
        .filter_entry(|dir_entry| {
            let is_file = dir_entry
                .file_type().is_some_and(|file_type| file_type.is_file());
            dbg!(&dir_entry, is_file);

            is_file
        })
        .build().for_each(|filtered_entry| {
            dbg!(&filtered_entry);
        });
}
