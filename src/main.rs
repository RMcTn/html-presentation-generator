use std::fs::*;
use std::io::*;

const END_PAGE_PATH: &str = "end.html";

fn main() {
    // TODO(reece): Link a CSS File
    // TODO(reece): Better layout
    // TODO(reece): get file name from argument
    let contents = std::fs::read_to_string("stuff.txt").unwrap();
    let mut lines: Vec<&str> = contents.split('\n').collect();

    chomp_trailing_empty_lines(&mut lines);

    let mut page = 1;

    let mut in_a_page = false;
    let mut current_file = File::create("page_0.html").unwrap();
    for i in 0..lines.len() {
        let line = &lines[i];
        if line.is_empty() {
            writeln!(current_file, "</ul>").unwrap();
            page += 1;
            in_a_page = false;
        } else {
            if !in_a_page {
                in_a_page = true;
                let new_file_path = format!("page_{}.html", page);
                let previous_page_link = format!("page_{}.html", page - 1);
                let next_page_link = format!("page_{}.html", page + 1);
                current_file = File::create(new_file_path).unwrap();
                if page > 0 {
                    writeln!(
                        current_file,
                        "<a href='{}'>previous</a>",
                        previous_page_link
                    )
                    .unwrap();
                }

                let last_line = lines.last().unwrap();

                if is_last_page(&lines, i, last_line) {
                    writeln!(current_file, "<a href='{}'>next</a>", END_PAGE_PATH).unwrap();
                } else {
                    writeln!(current_file, "<a href='{}'>next</a>", next_page_link).unwrap();
                }
                writeln!(current_file, "<br>").unwrap();
                writeln!(current_file, "<ul>").unwrap();
            }
            writeln!(current_file, "<li>{}</li>", line).unwrap();
        }
    }
    let mut end_page = File::create(END_PAGE_PATH).unwrap();

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

fn is_last_page(lines: &Vec<&str>, current_line_index: usize, last_line: &str) -> bool {
    let mut is_last_page = false;
    for j in current_line_index..lines.len() {
        let next_line = &lines[j];
        if next_line.is_empty() {
            break;
        }
        if *next_line == last_line {
            is_last_page = true;
            break;
        }
    }
    is_last_page
}
