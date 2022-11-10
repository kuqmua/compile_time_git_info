use std::{
    fs::File,
    io::{BufReader, Read},
    path::Path,
};

#[proc_macro_derive(CompileTimeGitInfoTufaClient)]
pub fn derive_compile_time_git_info_tufa_client(
    _input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    generate(
        // input,
        "tufa_client",
    )
}

#[proc_macro_derive(CompileTimeGitInfoTufaCommon)]
pub fn derive_compile_time_git_info_tufa_common(
    _input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    generate(
        // input,
        "tufa_common",
    )
}

#[proc_macro_derive(CompileTimeGitInfoTufaGrpcClient)]
pub fn derive_compile_time_git_info_tufa_grpc_client(
    _input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    generate(
        // input,
        "tufa_grpc_client",
    )
}

#[proc_macro_derive(CompileTimeGitInfoTufaGrpcServer)]
pub fn derive_compile_time_git_info_tufa_grpc_server(
    _input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    generate(
        // input,
        "tufa_grpc_server",
    )
}

#[proc_macro_derive(CompileTimeGitInfoTufaServer)]
pub fn derive_compile_time_git_info_tufa_server(
    _input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    generate(
        // input,
        "tufa_server",
    )
}

#[proc_macro_derive(CompileTimeGitInfoTufaTelegramBot)]
pub fn derive_compile_time_git_info_tufa_telegram_bot(
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    generate(
        // input,
        "tufa_telegram_bot",
    )
}

fn generate(
    // input: proc_macro::TokenStream,
    repo_name: &str,
) -> proc_macro::TokenStream {
    //it only supported for enums without values
    // let ast: syn::DeriveInput =
    //     syn::parse(input).expect("CompileTimeGitInfo syn::parse(input) failed");
    // let ident = &ast.ident;
    // let commit_id_token_stream = format!("\"kekw\"")
    //     .parse::<proc_macro2::TokenStream>()
    //     .expect("path parse failed");
    //
    // let path: String;
    // let git_folder_name = ".git";
    let first_guess = format!("../.git/modules/src/{}/", repo_name);
    let path: String = if Path::new(&first_guess).is_dir() {
        first_guess
    } else {
        panic!("its not a dir");
    };
    let full_path = &format!("{}{}", path, "logs/HEAD");
    let file = File::open(Path::new(full_path))
        .unwrap_or_else(|e| panic!("cannot open HEAD file, error: \"{}\"", e));
    let mut buf_reader = BufReader::new(file);
    let mut git_logs_head_content = String::new();
    buf_reader
        .read_to_string(&mut git_logs_head_content)
        .unwrap_or_else(|e| panic!("cannot read to string from HEAD file, error: \"{}\"", e));
    let from_handle = "from ";
    let from_handle_index = git_logs_head_content
        .find(from_handle)
        .unwrap_or_else(|| panic!("no \"{}\" inside git_logs_head_content", from_handle));
    let git_extenstion_name = ".git";
    let dot_git_index = git_logs_head_content
        .find(git_extenstion_name)
        .unwrap_or_else(|| {
            panic!(
                "no \"{}\" inside git_logs_head_content",
                git_extenstion_name
            )
        });
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
    let commit_id_token_stream = format!("\"{}\"", commit_id)
        .parse::<proc_macro2::TokenStream>()
        .expect("commit_id parse failed");
    let author = line_parts
        .get(2)
        .unwrap_or_else(|| panic!("failed to get 2 element from line_parts as author"))
        .to_string();
    let author_token_stream = format!("\"{}\"", author)
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
    let author_email_token_stream = format!("\"{}\"", author_email)
        .parse::<proc_macro2::TokenStream>()
        .expect("author_email parse failed");
    let commit_unix_time = line_parts
        .get(4)
        .unwrap_or_else(|| panic!("failed to get 4 element from line_parts as commit_unix_time"))
        .to_string();
    let commit_unix_time_token_stream = format!("\"{}\"", commit_unix_time)
        .parse::<proc_macro2::TokenStream>()
        .expect("path parse failed");
    let commit_unix_time_index = last_head_file_line
        .find(&commit_unix_time)
        .unwrap_or_else(|| {
            panic!(
                "cannot find \"{}\" for the second time inside {}",
                commit_unix_time, git_logs_head_content
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
        .unwrap_or_else(|| {
            panic!(
                "no \"{}\" inside \"{}\"",
                backslash_t, part_after_commit_unix_time
            )
        });
    let message = part_after_commit_unix_time
        .get(backslash_t_index + 1..)
        .unwrap_or_else(|| {
            panic!("failed to get slice from part_after_commit_unix_time as message")
        })
        .to_string();
    let message_token_stream = format!("\"{}\"", message)
        .parse::<proc_macro2::TokenStream>()
        .expect("path parse failed");
    let timezone = part_after_commit_unix_time
        .get(..backslash_t_index)
        .unwrap_or_else(|| {
            panic!("failed to get slice from part_after_commit_unix_time as timezone")
        })
        .to_string();
    let timezone_token_stream = format!("\"{}\"", timezone)
        .parse::<proc_macro2::TokenStream>()
        .expect("path parse failed");
    let gen = quote::quote! {
        impl Kekw<'_> {
            pub fn get_git_commit_info<'a>() -> Kekw<'a> {
                Kekw {
                    commit_id: #commit_id_token_stream ,
                    repo_link: #repo_link_token_stream ,
                    author: #author_token_stream ,
                    author_email: #author_email_token_stream ,
                    commit_unix_time: #commit_unix_time_token_stream ,
                    timezone: #timezone_token_stream ,
                    message: #message_token_stream ,
                }
            }
        }
    };
    gen.into()
}
