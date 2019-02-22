mod Lexer;




#[cfg(test)]
mod tests {
    use super::Lexer::TOKEN;
    use super::Lexer::Lexer;

    #[test]
    fn it_initializes_new_lexer() {
        let source = String::from("<body><div><p>This is a paragraph</p></div></body>");
        let lexer = Lexer::new(&source);
        assert_eq!(lexer.source.collect::<String>(), source);
    }

    #[test]
    fn it_creates_the_correct_tokens() {
        let source = String::from("
        <!DOCTYPE html>
        <html>
            <head>
                <link href=\"css/styles.css\" rel=\"stylesheet\" />
            </head>
            <body>
                <div>
                    <p hidden class=\"center\">This is a paragraph</p>
                </div>
            </body>
        </html>
        ");
        let mut lexer = Lexer::new(&source);
        let answers = vec![
            TOKEN::TAG_START("!DOCTYPE".to_string()),
            TOKEN::BOOL_ATTR("html".to_string()),
            TOKEN::TAG_END,
            TOKEN::TAG_START("html".to_string()),
            TOKEN::TAG_END,
            TOKEN::TAG_START("head".to_string()),
            TOKEN::TAG_END,
            TOKEN::TAG_START("link".to_string()),
            TOKEN::ATTR(("href".to_string(), "css/styles.css".to_string())),
            TOKEN::ATTR(("rel".to_string(), "stylesheet".to_string())),
            TOKEN::SINGLE_TAG_END,
            TOKEN::END_TAG_START("head".to_string()),
            TOKEN::TAG_END,
            TOKEN::TAG_START("body".to_string()),
            TOKEN::TAG_END,
            TOKEN::TAG_START("div".to_string()),
            TOKEN::TAG_END,
            TOKEN::TAG_START("p".to_string()),
            TOKEN::BOOL_ATTR("hidden".to_string()),
            TOKEN::ATTR(("class".to_string(), "center".to_string())),
            TOKEN::TAG_END,
            TOKEN::TEXT("This is a paragraph".to_string()),
            TOKEN::END_TAG_START("p".to_string()),
            TOKEN::TAG_END,
            TOKEN::END_TAG_START("div".to_string()),
            TOKEN::TAG_END,
            TOKEN::END_TAG_START("body".to_string()),
            TOKEN::TAG_END,
            TOKEN::END_TAG_START("html".to_string()),
            TOKEN::TAG_END
        ];
        let mut i = 0;
        while let Some(cur_token) = lexer.next_token() {
            println!("{:?}", cur_token);
            assert_eq!(cur_token, answers[i]);
            i += 1;
        }
    }
}
