// Copyright 2019 MaidSafe.net limited.
//
// This SAFE Network Software is licensed to you under The General Public License (GPL), version 3.
// Unless required by applicable law or agreed to in writing, the SAFE Network Software distributed
// under the GPL Licence is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied. Please review the Licences for the specific language governing
// permissions and limitations relating to use of the SAFE Network Software.

use super::OutputFmt;
use log::debug;
use prettytable::{format::FormatBuilder, Table};
use safe_api::XorName;
use serde::ser::Serialize;
use std::collections::BTreeMap;
use std::io::{self, stdin, stdout, Write};

// Warn the user about a dry-run being performed
pub fn notice_dry_run() {
    println!("NOTE the operation is being performed in dry-run mode, therefore no changes are committed to the network.");
}

// Converts the XOR name bytes into a hex encoded string
pub fn xorname_to_hex(xorname: &XorName) -> String {
    xorname.0.iter().map(|b| format!("{:02x}", b)).collect()
}

// Read the target location from the STDIN if is not an arg provided
pub fn get_from_arg_or_stdin(
    target_arg: Option<String>,
    message: Option<&str>,
) -> Result<String, String> {
    let the_message = message.unwrap_or_else(|| "...awaiting data from STDIN stream...");
    match target_arg {
        Some(t) => Ok(t),
        None => {
            println!("{}", &the_message);
            let mut input = String::new();
            match io::stdin().read_line(&mut input) {
                Ok(n) => {
                    debug!("Read ({} bytes) from STDIN: {}", n, input);
                    input.truncate(input.len() - 1);
                    Ok(input)
                }
                Err(_) => Err("Failed to read from STDIN stream".to_string()),
            }
        }
    }
}

// Prompt the user with the message provided
pub fn prompt_user(prompt_msg: &str, error_msg: &str) -> Result<String, String> {
    let mut user_input = String::new();
    print!("{}", prompt_msg);
    let _ = stdout().flush();
    stdin().read_line(&mut user_input).map_err(|_| error_msg)?;
    if let Some('\n') = user_input.chars().next_back() {
        user_input.pop();
    }
    if let Some('\r') = user_input.chars().next_back() {
        user_input.pop();
    }

    if user_input.is_empty() {
        Err(error_msg.to_string())
    } else {
        Ok(user_input)
    }
}

// Unwrap secret key string provided, otherwise prompt user to provide it
pub fn get_secret_key(key_xorurl: &str, sk: Option<String>, msg: &str) -> Result<String, String> {
    let mut sk = sk.unwrap_or_else(|| String::from(""));

    if sk.is_empty() {
        let msg = if key_xorurl.is_empty() {
            format!("Enter secret key corresponding to {}: ", msg)
        } else {
            format!(
                "Enter secret key corresponding to public key at \"{}\": ",
                key_xorurl
            )
        };
        sk = prompt_user(&msg, "Invalid input")?;
    }

    Ok(sk)
}

pub fn parse_tx_id(src: &str) -> Result<u64, String> {
    src.parse::<u64>()
        .map_err(|err| format!("{}. A valid TX Id is a number between 0 and 2^64", err))
}

pub fn gen_processed_files_table(
    processed_files: &BTreeMap<String, (String, String)>,
    show_change_sign: bool,
) -> (Table, u64) {
    let mut table = Table::new();
    let format = FormatBuilder::new()
        .column_separator(' ')
        .padding(0, 1)
        .build();
    table.set_format(format);
    let mut success_count = 0;
    for (file_name, (change, link)) in processed_files.iter() {
        if change != "E" {
            success_count += 1;
        }
        if show_change_sign {
            table.add_row(row![change, file_name, link]);
        } else {
            table.add_row(row![file_name, link]);
        }
    }
    (table, success_count)
}

// serialize structured value using any format from OutputFmt
// except OutputFmt::Pretty, which must be handled by caller.
pub fn serialise_output<T: ?Sized>(value: &T, fmt: OutputFmt) -> String
where
    T: Serialize,
{
    match fmt {
        OutputFmt::Yaml => serde_yaml::to_string(&value)
            .unwrap_or_else(|_| "Failed to serialise output to yaml".to_string()),
        OutputFmt::Json => serde_json::to_string_pretty(&value)
            .unwrap_or_else(|_| "Failed to serialise output to json".to_string()),
        OutputFmt::JsonCompact => serde_json::to_string(&value)
            .unwrap_or_else(|_| "Failed to serialise output to json".to_string()),
        OutputFmt::Pretty => {
            "OutputFmt::Pretty' not handled by caller, in serialise_output()".to_string()
        }
    }
}
