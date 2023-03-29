use std::env;
use std::fs::*;
use std::io::*;

const END_PAGE_PATH: &str = "end.html";
const CSS_FILE_PATH: &str = "styles.css";

fn main() {
    // How long till this just becomes a HTML template language

    let args: Vec<String> = env::args().collect();
    dbg!(&args);
    if args.len() != 2 {
        // TODO(reece): Update this message with an actual name
        eprintln!("USAGE: cargo run <path_to_file>");
        return;
    }

    // TODO(reece): Better layout
    // TODO(reece): add images
    let contents = std::fs::read_to_string(&args[1]).unwrap();
    let mut lines: Vec<&str> = contents.split('\n').collect();

    chomp_trailing_empty_lines(&mut lines);

    let mut page = 1;

    let mut in_a_page = false;
    let mut current_file = File::create("page_1.html").unwrap();
    for i in 0..lines.len() {
        let line = &lines[i];
        if line.is_empty() {
            writeln!(current_file, "</ul>").unwrap();
            writeln!(current_file, "</div>").unwrap();
            page += 1;
            in_a_page = false;
        } else {
            if !in_a_page {
                in_a_page = true;
                let is_last_page = is_last_page(&lines, i);
                current_file = create_html_page(page, is_last_page).unwrap();
            }
            writeln!(current_file, "<li>{}</li>", line).unwrap();
        }
    }
    writeln!(current_file, "</ul>").unwrap();
    let mut end_page = File::create(END_PAGE_PATH).unwrap();
    writeln!(
        end_page,
        "<head><link rel='stylesheet' href='{}'></head>",
        CSS_FILE_PATH
    )
    .unwrap();

    let last_page_link = format!("page_{}.html", page);
    writeln!(end_page, "<a href='{}'>previous</a>", last_page_link).unwrap();
    writeln!(end_page, "<h1>FIN</h1>").unwrap();

    println!("Wrote out {} pages", page);
}

fn chomp_trailing_empty_lines(lines: &mut Vec<&str>) {
    let mut lines_to_keep = lines.len();
    for line in lines.iter().rev() {
        if line.is_empty() {
            lines_to_keep -= 1;
        } else {
            break;
        }
    }
    lines.truncate(lines_to_keep);
}

fn is_last_page(lines: &Vec<&str>, current_line_index: usize) -> bool {
    let mut is_last_page = false;
    for j in current_line_index..lines.len() {
        let next_line = &lines[j];
        if next_line.is_empty() {
            break;
        }
        let last_line = lines.last().unwrap();
        if *next_line == *last_line {
            is_last_page = true;
            break;
        }
    }
    is_last_page
}

fn create_html_page(page: u32, is_last_page: bool) -> std::io::Result<File> {
    let new_file_path = format!("page_{}.html", page);
    let previous_page_link = format!("page_{}.html", page - 1);
    let next_page_link = format!("page_{}.html", page + 1);

    let mut current_file = File::create(new_file_path)?;
    writeln!(
        current_file,
        "<head><link rel='stylesheet' href='{}'></head>",
        CSS_FILE_PATH
    )
    .unwrap();
    if page > 1 {
        writeln!(
            current_file,
            "<a href='{}'>previous</a>",
            previous_page_link
        )
        .unwrap();
    }

    if is_last_page {
        writeln!(current_file, "<a href='{}'>next</a>", END_PAGE_PATH).unwrap();
    } else {
        writeln!(current_file, "<a href='{}'>next</a>", next_page_link).unwrap();
    }
    writeln!(current_file, "<br>").unwrap();
    writeln!(current_file, "<div class='content'>").unwrap();
    writeln!(current_file, "<ul>").unwrap();
    return Ok(current_file);
}
