#[derive(Debug)]
enum Token<'a> {
    Word(&'a str),
    RedirectStdout,
    RedirectStdin,
    Pipe,
}

fn next_token<'a>(s: &mut &'a str) -> Option<Token<'a>> {
    *s = s.trim_start();
    if s.is_empty() {
        return None;
    }
    if let Some(rem) = s.strip_prefix('>') {
        *s = rem;
        Some(Token::RedirectStdout)
    } else if let Some(rem) = s.strip_prefix('<') {
        *s = rem;
        Some(Token::RedirectStdin)
    } else if let Some(rem) = s.strip_prefix('|') {
        *s = rem;
        Some(Token::Pipe)
    } else {
        for (i, c) in s.char_indices() {
            if c.is_whitespace() || c == '|' || c == '<' || c == '>' {
                let word;
                (word, *s) = s.split_at(i);
                return Some(Token::Word(word));
            }
        }
        let word = *s;
        *s = "";
        Some(Token::Word(word))
    }
}

fn print_all_tokens(mut s: &str) {
    while let Some(token) = next_token(&mut s) {
        println!("{token:?}");
    }
}

fn main() {
    if ftkit::ARGS.len() != 2 {
        eprintln!("error: exactly one argument expected");
        return;
    }

    print_all_tokens(&ftkit::ARGS[1]);
}
