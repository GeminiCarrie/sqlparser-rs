// Copyright (C) 2017-2018 Baidu, Inc. All Rights Reserved.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions
// are met:
//
//  * Redistributions of source code must retain the above copyright
//    notice, this list of conditions and the following disclaimer.
//  * Redistributions in binary form must reproduce the above copyright
//    notice, this list of conditions and the following disclaimer in
//    the documentation and/or other materials provided with the
//    distribution.
//  * Neither the name of Baidu, Inc., nor the names of its
//    contributors may be used to endorse or promote products derived
//    from this software without specific prior written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
// "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
// LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
// A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
// OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
// SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
// LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
// DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
// THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
// (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
// OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

#![crate_name = "helloworldsampleenclave"]
#![crate_type = "staticlib"]

#![cfg_attr(not(target_env = "sgx"), no_std)]
#![cfg_attr(target_env = "sgx", feature(rustc_private))]

extern crate sgx_types;
#[cfg(not(target_env = "sgx"))]
#[macro_use]
extern crate sgx_tstd as std;
extern crate sgx_tunittest;

use sgx_types::*;
use std::string::String;
use std::vec::Vec;
use std::io::{self, Write};
use std::slice;
use sgx_tunittest::*;

extern crate sqlparser;

mod sqlparser_common;
mod sqlparser_mssql;
mod sqlparser_mysql;
mod sqlparser_postgres;

#[no_mangle]
pub extern "C" fn say_something(some_string: *const u8, some_len: usize) -> sgx_status_t {

    let str_slice = unsafe { slice::from_raw_parts(some_string, some_len) };
    let _ = io::stdout().write(str_slice);

    // A sample &'static string
    let rust_raw_string = "This is a in-Enclave ";
    // An array
    let word:[u8;4] = [82, 117, 115, 116];
    // An vector
    let word_vec:Vec<u8> = vec![32, 115, 116, 114, 105, 110, 103, 33];

    // Construct a string from &'static string
    let mut hello_string = String::from(rust_raw_string);

    // Iterate on word array
    for c in word.iter() {
        hello_string.push(*c as char);
    }

    // Rust style convertion
    hello_string += String::from_utf8(word_vec).expect("Invalid UTF-8")
                                               .as_str();

    // Ocall to normal world for output
    println!("{}", &hello_string);
    rsgx_unit_tests!(
sqlparser_mssql::ms,
sqlparser_mssql::ms_and_generic,
sqlparser_mssql::parse_mssql_apply_join,
sqlparser_mssql::parse_mssql_delimited_identifiers,
sqlparser_mssql::parse_mssql_identifiers,
sqlparser_mssql::parse_mssql_single_quoted_aliases,
sqlparser_mysql::mysql,
sqlparser_mysql::mysql_and_generic,
sqlparser_mysql::parse_identifiers,
sqlparser_mysql::parse_show_columns,
sqlparser_postgres::parse_copy_example,
sqlparser_postgres::parse_create_table_from_pg_dump,
sqlparser_postgres::parse_create_table_with_defaults,
sqlparser_postgres::parse_create_table_with_inherit,
sqlparser_postgres::parse_set,
sqlparser_postgres::parse_show,
sqlparser_postgres::pg,
sqlparser_postgres::pg_and_generic,
sqlparser_common::lateral_derived,
sqlparser_common::parse_aggregate_with_group_by,
sqlparser_common::parse_alter_table_constraints,
sqlparser_common::parse_bad_constraint,
sqlparser_common::parse_between,
sqlparser_common::parse_between_with_expr,
sqlparser_common::parse_cast,
sqlparser_common::parse_column_aliases,
sqlparser_common::parse_commit,
sqlparser_common::parse_complex_join,
sqlparser_common::parse_compound_expr_1,
sqlparser_common::parse_compound_expr_2,
sqlparser_common::parse_count_wildcard,
sqlparser_common::parse_create_external_table,
sqlparser_common::parse_create_materialized_view,
sqlparser_common::parse_create_table,
sqlparser_common::parse_create_table_empty,
sqlparser_common::parse_create_table_trailing_comma,
sqlparser_common::parse_create_table_with_options,
sqlparser_common::parse_create_view,
sqlparser_common::parse_create_view_with_columns,
sqlparser_common::parse_create_view_with_options,
sqlparser_common::parse_cross_join,
sqlparser_common::parse_cte_renamed_columns,
sqlparser_common::parse_ctes,
sqlparser_common::parse_delete_statement,
sqlparser_common::parse_delimited_identifiers,
sqlparser_common::parse_derived_tables,
sqlparser_common::parse_drop_table,
sqlparser_common::parse_drop_view,
sqlparser_common::parse_escaped_single_quote_string_predicate,
sqlparser_common::parse_exists_subquery,
sqlparser_common::parse_extract,
sqlparser_common::parse_fetch,
sqlparser_common::parse_fetch_variations,
sqlparser_common::parse_from_advanced,
sqlparser_common::parse_implicit_join,
sqlparser_common::parse_in_list,
sqlparser_common::parse_in_subquery,
sqlparser_common::parse_insert_invalid,
sqlparser_common::parse_insert_values,
sqlparser_common::parse_invalid_infix_not,
sqlparser_common::parse_invalid_subquery_without_parens,
sqlparser_common::parse_invalid_table_name,
sqlparser_common::parse_is_not_null,
sqlparser_common::parse_is_null,
sqlparser_common::parse_join_nesting,
sqlparser_common::parse_join_syntax_variants,

                     );

    sgx_status_t::SGX_SUCCESS
}
