use super::*;
use strong_xml::xmlparser::*;
use strong_xml::*;

impl<'a> XmlRead<'a> for Script {
    fn from_reader(reader: &mut XmlReader<'a>) -> XmlResult<Self> {
        let mut body = String::new();
        loop {
            if let Some(token) = reader.next() {
                match token? {
                    Token::Text { text, .. } | Token::Cdata { text, .. } => {
                        body.push_str(text.as_str());
                    }
                    Token::ElementEnd {
                        end: ElementEnd::Open,
                        ..
                    } => {}
                    Token::ElementEnd { .. } => {
                        break;
                    }
                    _ => {}
                }
            }
        }
        if body.is_empty() {
            Ok(Script {
                ..Default::default()
            })
        } else {
            Ok(Script {
                content: Some(body),
            })
        }
    }
}
