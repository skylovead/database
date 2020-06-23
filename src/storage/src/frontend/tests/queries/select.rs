// Copyright 2020 Alex Dukhno
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use super::*;
use sql_types::SqlType;

#[test]
fn select_from_table_that_does_not_exist() {
    let mut storage = FrontendStorage::default().expect("no system errors");

    create_schema(&mut storage, "schema_name");
    let table_columns = storage
        .table_columns("schema_name", "not_existed")
        .expect("no system errors")
        .expect("columns")
        .into_iter()
        .map(|(name, _sql_type)| name)
        .collect();

    assert_eq!(
        storage
            .select_all_from("schema_name", "not_existed", table_columns)
            .expect("no system errors"),
        Err(OperationOnTableError::TableDoesNotExist)
    );
}

#[test]
fn select_all_from_table_with_many_columns() {
    let mut storage = FrontendStorage::default().expect("no system errors");

    create_schema_with_table(
        &mut storage,
        "schema_name",
        "table_name",
        vec![
            ("column_1", SqlType::SmallInt),
            ("column_2", SqlType::SmallInt),
            ("column_3", SqlType::SmallInt),
        ],
    );

    insert_into(&mut storage, "schema_name", "table_name", vec!["1", "2", "3"]);

    let table_columns = storage
        .table_columns("schema_name", "table_name")
        .expect("no system errors")
        .expect("table has columns")
        .into_iter()
        .map(|(name, _sql_type)| name)
        .collect();

    assert_eq!(
        storage
            .select_all_from("schema_name", "table_name", table_columns)
            .expect("no system errors"),
        Ok((
            vec![
                ("column_1".to_owned(), SqlType::SmallInt),
                ("column_2".to_owned(), SqlType::SmallInt),
                ("column_3".to_owned(), SqlType::SmallInt)
            ],
            vec![vec!["1".to_owned(), "2".to_owned(), "3".to_owned()]]
        ))
    );
}

#[test]
fn select_first_and_last_columns_from_table_with_multiple_columns() {
    let mut storage = FrontendStorage::default().expect("no system errors");

    create_schema_with_table(
        &mut storage,
        "schema_name",
        "table_name",
        vec![
            ("first", SqlType::SmallInt),
            ("middle", SqlType::SmallInt),
            ("last", SqlType::SmallInt),
        ],
    );

    insert_into(&mut storage, "schema_name", "table_name", vec!["1", "2", "3"]);
    insert_into(&mut storage, "schema_name", "table_name", vec!["4", "5", "6"]);
    insert_into(&mut storage, "schema_name", "table_name", vec!["7", "8", "9"]);

    assert_eq!(
        storage
            .select_all_from("schema_name", "table_name", vec!["first".to_owned(), "last".to_owned()])
            .expect("no system errors"),
        Ok((
            vec![
                ("first".to_owned(), SqlType::SmallInt),
                ("last".to_owned(), SqlType::SmallInt)
            ],
            vec![
                vec!["1".to_owned(), "3".to_owned()],
                vec!["4".to_owned(), "6".to_owned()],
                vec!["7".to_owned(), "9".to_owned()],
            ],
        ))
    );
}

#[test]
fn select_all_columns_reordered_from_table_with_multiple_columns() {
    let mut storage = FrontendStorage::default().expect("no system errors");

    create_schema_with_table(
        &mut storage,
        "schema_name",
        "table_name",
        vec![
            ("first", SqlType::SmallInt),
            ("middle", SqlType::SmallInt),
            ("last", SqlType::SmallInt),
        ],
    );

    insert_into(&mut storage, "schema_name", "table_name", vec!["1", "2", "3"]);
    insert_into(&mut storage, "schema_name", "table_name", vec!["4", "5", "6"]);
    insert_into(&mut storage, "schema_name", "table_name", vec!["7", "8", "9"]);

    assert_eq!(
        storage
            .select_all_from(
                "schema_name",
                "table_name",
                vec!["last".to_owned(), "first".to_owned(), "middle".to_owned()]
            )
            .expect("no system errors"),
        Ok((
            vec![
                ("last".to_owned(), SqlType::SmallInt),
                ("first".to_owned(), SqlType::SmallInt),
                ("middle".to_owned(), SqlType::SmallInt)
            ],
            vec![
                vec!["3".to_owned(), "1".to_owned(), "2".to_owned()],
                vec!["6".to_owned(), "4".to_owned(), "5".to_owned()],
                vec!["9".to_owned(), "7".to_owned(), "8".to_owned()],
            ],
        ))
    );
}

#[test]
fn select_with_column_name_duplication() {
    let mut storage = FrontendStorage::default().expect("no system errors");

    create_schema_with_table(
        &mut storage,
        "schema_name",
        "table_name",
        vec![
            ("first", SqlType::SmallInt),
            ("middle", SqlType::SmallInt),
            ("last", SqlType::SmallInt),
        ],
    );

    insert_into(&mut storage, "schema_name", "table_name", vec!["1", "2", "3"]);
    insert_into(&mut storage, "schema_name", "table_name", vec!["4", "5", "6"]);
    insert_into(&mut storage, "schema_name", "table_name", vec!["7", "8", "9"]);

    assert_eq!(
        storage
            .select_all_from(
                "schema_name",
                "table_name",
                vec![
                    "last".to_owned(),
                    "middle".to_owned(),
                    "first".to_owned(),
                    "last".to_owned(),
                    "middle".to_owned()
                ]
            )
            .expect("no system errors"),
        Ok((
            vec![
                ("last".to_owned(), SqlType::SmallInt),
                ("middle".to_owned(), SqlType::SmallInt),
                ("first".to_owned(), SqlType::SmallInt),
                ("last".to_owned(), SqlType::SmallInt),
                ("middle".to_owned(), SqlType::SmallInt)
            ],
            vec![
                vec![
                    "3".to_owned(),
                    "2".to_owned(),
                    "1".to_owned(),
                    "3".to_owned(),
                    "2".to_owned()
                ],
                vec![
                    "6".to_owned(),
                    "5".to_owned(),
                    "4".to_owned(),
                    "6".to_owned(),
                    "5".to_owned()
                ],
                vec![
                    "9".to_owned(),
                    "8".to_owned(),
                    "7".to_owned(),
                    "9".to_owned(),
                    "8".to_owned()
                ],
            ],
        ))
    );
}
