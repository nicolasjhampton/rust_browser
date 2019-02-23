#![allow(dead_code)]

#[derive(Debug)]
#[derive(PartialEq)]
pub enum TOKEN {
    TAG_START(String),
    END_TAG_START(String),
    TAG_END,
    SINGLE_TAG_END,
    ATTR((String, String)),
    BOOL_ATTR(String),
    TEXT(String)
}