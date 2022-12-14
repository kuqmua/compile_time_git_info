#![deny(
    clippy::indexing_slicing,
    clippy::integer_arithmetic,
    clippy::unwrap_used,
    clippy::float_arithmetic
)]
#![allow(clippy::too_many_arguments)]

use std::{
    fs::File,
    io::{BufReader, Read},
    path::Path,
};

#[proc_macro_attribute]
pub fn generate_const_git_information(
    repo_name_and_git_info_path: proc_macro::TokenStream,
    _item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let repo_name_and_path_as_string = repo_name_and_git_info_path.to_string();
    let splitted = &repo_name_and_path_as_string
        .split(", ")
        .collect::<Vec<&str>>();
    let (repo_name, git_info_path) = match splitted.len() == 2 {
        false => panic!("arguments length is not 2"),
        true => {
            if !(splitted[1] == "crate" || splitted[1] == "tufa_common") {
                panic!("plitted[1] !== crate or tufa_common");
            }
            //maybe add repo name checks?
            (splitted[0], splitted[1])
        }
    };
    let path = format!("../.git/modules/src/{repo_name}/");
    let path: String = if Path::new(&path).is_dir() {
        path
    } else {
        panic!("{path} is not a dir");
    };
    let full_path = &format!("{}{}", path, "logs/HEAD");
    let file = File::open(Path::new(full_path))
        .unwrap_or_else(|e| panic!("cannot open logs/HEAD file, error: \"{e}\""));
    let mut buf_reader = BufReader::new(file);
    let mut git_logs_head_content = String::new();
    buf_reader
        .read_to_string(&mut git_logs_head_content)
        .unwrap_or_else(|e| panic!("cannot read_to_string from HEAD file, error: \"{e}\""));
    let from_handle = "from ";
    let from_handle_index = git_logs_head_content
        .find(from_handle)
        .unwrap_or_else(|| panic!("no \"{from_handle}\" inside git_logs_head_content"));
    let git_extenstion_name = ".git";
    let dot_git_index = git_logs_head_content
        .find(git_extenstion_name)
        .unwrap_or_else(|| panic!("no \"{git_extenstion_name}\" inside git_logs_head_content"));
    let repo_link_token_stream = git_logs_head_content
        .get(from_handle_index + from_handle.len()..dot_git_index)
        .unwrap_or_else(|| panic!("failed to get slice from git_logs_head_content"))
        .to_string();
    let head_file_lines: Vec<&str> = git_logs_head_content.lines().collect::<Vec<&str>>();
    let last_head_file_line = head_file_lines
        .last()
        .expect("no last element inside git head file lines");
    let line_parts: Vec<&str> = last_head_file_line.split(' ').collect();
    let commit_id = line_parts
        .get(1)
        .unwrap_or_else(|| panic!("failed to get 1 element from line_parts as commit_id"))
        .to_string();
    let commit_id_replaced = commit_id.replace('"', "\\\""); //bad, bad decision
    let commit_id_token_stream = format!("\"{commit_id_replaced}\"")
        .parse::<proc_macro2::TokenStream>()
        .expect("commit_id parse failed");
    let author = line_parts
        .get(2)
        .unwrap_or_else(|| panic!("failed to get 2 element from line_parts as author"))
        .to_string();
    let author_replaced = author.replace('"', "\\\""); //bad, bad decision
    let author_token_stream = format!("\"{author_replaced}\"")
        .parse::<proc_macro2::TokenStream>()
        .expect("author parse failed");
    let unhandled_author_email = line_parts
        .get(3)
        .unwrap_or_else(|| {
            panic!("failed to get 3 element from line_parts as slice for author_email")
        })
        .to_string();
    let author_email = unhandled_author_email
        .get(1..unhandled_author_email.len() - 1)
        .unwrap_or_else(|| panic!("failed to get slice from line_parts as author_email"))
        .to_string();
    let author_email_replaced = author_email.replace('"', "\\\""); //bad, bad decision
    let author_email_token_stream = format!("\"{author_email_replaced}\"")
        .parse::<proc_macro2::TokenStream>()
        .expect("author_email parse failed");
    let commit_unix_time = line_parts
        .get(4)
        .unwrap_or_else(|| panic!("failed to get 4 element from line_parts as commit_unix_time"))
        .to_string();
    let commit_unix_time_replaced = commit_unix_time.replace('"', "\\\""); //bad, bad decision
    let commit_unix_time_token_stream = format!("\"{commit_unix_time_replaced}\"")
        .parse::<proc_macro2::TokenStream>()
        .expect("path parse failed");
    let commit_unix_time_index = last_head_file_line
        .find(&commit_unix_time)
        .unwrap_or_else(|| {
            panic!(
                "cannot find \"{commit_unix_time}\" for the second time inside {git_logs_head_content}"
            )
        });
    let part_after_commit_unix_time = last_head_file_line
        .get(commit_unix_time_index + commit_unix_time.len() + 1..)
        .unwrap_or_else(|| {
            panic!("failed to get slice from last_head_file_line as part_after_commit_unix_time")
        })
        .to_string();
    let backslash_t = "\t";
    let backslash_t_index = part_after_commit_unix_time
        .find(backslash_t)
        .unwrap_or_else(|| panic!("no \"{backslash_t}\" inside \"{part_after_commit_unix_time}\""));
    let timezone = part_after_commit_unix_time
        .get(..backslash_t_index)
        .unwrap_or_else(|| {
            panic!("failed to get slice from part_after_commit_unix_time as timezone")
        })
        .to_string();
    let timezone_replaced = timezone.replace('"', "\\\""); //bad, bad decision
    let timezone_token_stream = format!("\"{timezone_replaced}\"")
        .parse::<proc_macro2::TokenStream>()
        .expect("path parse failed");
    let message = part_after_commit_unix_time
        .get(backslash_t_index + 1..)
        .unwrap_or_else(|| {
            panic!("failed to get slice from part_after_commit_unix_time as message")
        });
    let message_replaced = message.replace('"', "\\\""); //bad, bad decision
    let message_token_stream = format!("\"{message_replaced}\"")
        .parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| {
            panic!("failed to parse message_token_stream");
        });
    let path_to_git_info_token_stream =
        format!("{git_info_path}::common::git::git_info::GitInformation")
            .parse::<proc_macro2::TokenStream>()
            .expect("path_to_git_info parse failed");
    let gen = quote::quote! {
        pub static GIT_INFO: #path_to_git_info_token_stream = #path_to_git_info_token_stream {
            git_commit_id: #commit_id_token_stream ,
            git_repo_link: #repo_link_token_stream ,
            git_author: #author_token_stream ,
            git_author_email: #author_email_token_stream ,
            git_commit_unix_time: #commit_unix_time_token_stream ,
            git_timezone: #timezone_token_stream ,
            git_message: #message_token_stream ,
        };
    };
    gen.into()
}
