//! # XML token hacks
//!
//! ## Warning
//!
//! This module contains hacks and workaround for dealing with strong-xml/xmlparser and BPMN's
//! quirks. Internal use only.
use strong_xml::xmlparser::{ElementEnd, Token};
use strong_xml::{XmlError, XmlReader, XmlResult};

pub(crate) trait ReadSource {
    /// Reads an element to the end and returns the source of it, renamed.
    ///
    /// This is done so that we can use other XmlReaders that were generated
    /// for other tags.
    fn read_source_till_end(&mut self, rename: &str, to: &str) -> XmlResult<String>;
}

fn push_token(token: Token, s: &mut String, rename: &str, to: &str) {
    match token {
        Token::Declaration { span, .. } => s.push_str(span.as_str()),
        Token::ProcessingInstruction { span, .. } => s.push_str(span.as_str()),
        Token::Comment { span, .. } => s.push_str(span.as_str()),
        Token::DtdStart { span, .. } => s.push_str(span.as_str()),
        Token::EmptyDtd { span, .. } => s.push_str(span.as_str()),
        Token::EntityDeclaration { span, .. } => s.push_str(span.as_str()),
        Token::DtdEnd { span, .. } => s.push_str(span.as_str()),
        Token::ElementStart { prefix, local, .. } => {
            s.push('<');
            if !prefix.as_str().is_empty() {
                s.push_str(prefix.as_str());
                s.push(':');
            }
            if local.as_str() == rename {
                s.push_str(to);
            } else {
                s.push_str(local.as_str());
            }
            s.push(' ');
        }
        Token::Attribute { span, .. } => {
            s.push_str(span.as_str());
            s.push(' ');
        }
        Token::ElementEnd {
            end: ElementEnd::Close(prefix, local),
            ..
        } => {
            s.push_str("</");
            if !prefix.as_str().is_empty() {
                s.push_str(prefix.as_str());
                s.push(':');
            }
            if local.as_str() == rename {
                s.push_str(to);
            } else {
                s.push_str(local.as_str());
            }
            s.push('>');
        }
        Token::ElementEnd { span, .. } => s.push_str(span.as_str()),
        Token::Text { text, .. } => s.push_str(text.as_str()),
        Token::Cdata { span, .. } => s.push_str(span.as_str()),
    }
}

impl<'a> ReadSource for XmlReader<'a> {
    fn read_source_till_end(&mut self, rename: &str, to: &str) -> XmlResult<String> {
        let mut body = String::new();
        let mut tag = "".to_string();
        if let Some(start) = self.peek() {
            match start {
                Ok(Token::ElementStart { local, .. }) => {
                    tag.push_str(local.as_str());
                }
                token => {
                    return Err(XmlError::UnexpectedToken {
                        token: format!("{:?}", token),
                    });
                }
            }
        }
        let mut container = false;
        while let Some(token) = self.next() {
            match token? {
                Token::ElementEnd {
                    end: ElementEnd::Close(ns, local),
                    span,
                    ..
                } if local.as_str() == tag => {
                    push_token(
                        Token::ElementEnd {
                            end: ElementEnd::Close(ns, local),
                            span,
                        },
                        &mut body,
                        rename,
                        to,
                    );
                    return Ok(body);
                }
                token
                @
                Token::ElementEnd {
                    end: ElementEnd::Close(_, _),
                    ..
                } if !container => {
                    container = true;
                    push_token(token, &mut body, rename, to);
                }
                token
                @
                Token::ElementEnd {
                    end: ElementEnd::Empty,
                    ..
                } if !container => {
                    push_token(token, &mut body, rename, to);
                    return Ok(body);
                }
                token => {
                    push_token(token, &mut body, rename, to);
                }
            }
        }
        Ok(body)
    }
}
