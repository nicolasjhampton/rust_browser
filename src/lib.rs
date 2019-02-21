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
        <body>
            <div>
                <p>This is a paragraph</p>
            </div>
        </body>
        ");
        let mut lexer = Lexer::new(&source);
        let answers = vec![
            TOKEN::TAG_START,
            TOKEN::TAG_NAME("body".to_string()),
            TOKEN::TAG_END,
            TOKEN::TAG_START,
            TOKEN::TAG_NAME("div".to_string()),
            TOKEN::TAG_END,
            TOKEN::TAG_START,
            TOKEN::TAG_NAME("p".to_string()),
            TOKEN::TAG_END,
            TOKEN::TEXT("This is a paragraph".to_string()),
            TOKEN::END_TAG_START,
            TOKEN::TAG_NAME("p".to_string()),
            TOKEN::TAG_END,
            TOKEN::END_TAG_START,
            TOKEN::TAG_NAME("div".to_string()),
            TOKEN::TAG_END,
            TOKEN::END_TAG_START,
            TOKEN::TAG_NAME("body".to_string()),
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
