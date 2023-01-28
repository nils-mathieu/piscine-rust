fn print_justified(words: &[String], columns: usize) {
    let mut width = 0;
    for word in words {
        width += unicode_width::UnicodeWidthStr::width(word.as_str());
    }

    if words.len() == 1 {
        println!("{}", &words[0]);
        return;
    }

    let gap_count = words.len() - 1;
    let total_gaps = columns - width;
    let normal_gap = total_gaps / gap_count;
    let one_more = total_gaps % gap_count;

    if !words.is_empty() {
        print!("{}", &words[0]);
        for i in 0..gap_count {
            let mut gap_size = normal_gap;
            if i < one_more {
                gap_size += 1;
            }
            for _ in 0..gap_size {
                print!(" ");
            }
            print!("{}", &words[i + 1]);
        }
    }
    println!();
}

fn print_paragraph(words: &[String], columns: usize) {
    let mut start_of_line = 0;
    let mut end_of_line = 0;
    let mut total_width = 0;
    while end_of_line < words.len() {
        let word_width = unicode_width::UnicodeWidthStr::width(words[end_of_line].as_str()) + 1;
        if total_width + word_width - 1 <= columns {
            total_width += word_width;
            end_of_line += 1;
        } else if start_of_line == end_of_line {
            println!("{}", &words[start_of_line]);
            end_of_line += 1;
            start_of_line = end_of_line;
        } else {
            print_justified(&words[start_of_line..end_of_line], columns);
            total_width = 0;
            start_of_line = end_of_line;
        }
    }

    let rem = &words[start_of_line..end_of_line];
    if !rem.is_empty() {
        print!("{}", &rem[0]);
        for word in &rem[1..] {
            print!(" {}", word);
        }
    }
    println!();
}

fn main() {
    assert_eq!(ftkit::ARGS.len(), 2);
    let columns: usize = ftkit::ARGS[1].parse().unwrap();

    let mut words = Vec::new();
    let mut consecutive_newlines = 0;

    loop {
        let line = ftkit::read_line();

        if line.is_empty() {
            break;
        }

        if line.trim().is_empty() {
            consecutive_newlines += 1;
        } else {
            if consecutive_newlines >= 2 {
                print_paragraph(&words, columns);
                words.clear();
                println!();
            }
            consecutive_newlines = 0;
        }

        for word in line.split_whitespace() {
            words.push(word.to_string());
        }
    }

    print_paragraph(&words, columns);
}
