use super::*;
use serde::{Deserialize, Serialize};
use strong_xml::xmlparser::*;
use strong_xml::*;

/// Expression or Formal Expression
///
/// BPMN expressions are often hot-patched with `xsi:type`
/// to re-type them to formal expressions. This type handles this case.
#[derive(Hash, Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum Expr {
    Expression(Expression),
    FormalExpression(FormalExpression),
}

impl Default for Expr {
    fn default() -> Self {
        Self::Expression(Default::default())
    }
}

impl Expr {
    // Converts Expr to its inner element
    pub fn into_inner(self) -> Box<dyn DocumentElement> {
        match self {
            Expr::Expression(e) => Box::new(e) as Box<dyn DocumentElement>,
            Expr::FormalExpression(e) => Box::new(e) as Box<dyn DocumentElement>,
        }
    }
}

impl From<Expression> for Expr {
    fn from(e: Expression) -> Self {
        Self::Expression(e)
    }
}

impl From<FormalExpression> for Expr {
    fn from(e: FormalExpression) -> Self {
        Self::FormalExpression(e)
    }
}

#[cast_to]
impl DocumentElementContainer for Expr {
    fn find_by_id_mut(&mut self, id: &str) -> Option<&mut dyn DocumentElement> {
        match self {
            Expr::Expression(e) => e.find_by_id_mut(id),
            Expr::FormalExpression(e) => e.find_by_id_mut(id),
        }
    }

    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        match self {
            Expr::Expression(e) => e.find_by_id(id),
            Expr::FormalExpression(e) => e.find_by_id(id),
        }
    }
}
#[cast_to]
impl DocumentElement for Expr {
    fn element(&self) -> Element {
        match self {
            Expr::Expression(_) => Element::Expression,
            Expr::FormalExpression(_) => Element::FormalExpression,
        }
    }
}

impl<'a> XmlRead<'a> for Expr {
    fn from_reader(reader: &mut XmlReader<'a>) -> XmlResult<Self> {
        let mut attributes = vec![];
        let mut formal = false;
        loop {
            if let Some(token) = reader.peek() {
                match token {
                    Ok(Token::ElementStart { span: _, .. }) => {}
                    Ok(Token::Attribute {
                        prefix,
                        local,
                        value,
                        ..
                    }) if prefix.as_str() == "xsi"
                        && local.as_str() == "type"
                        && value.as_str() == "tFormalExpression" =>
                    {
                        formal = true;
                    }
                    Ok(a @ Token::Attribute { .. }) => {
                        attributes.push(*a);
                    }
                    Ok(Token::ElementEnd {
                        end: ElementEnd::Open,
                        ..
                    }) => {
                        // consume
                        let _ = reader.next();
                        break;
                    }
                    _ => {
                        break;
                    }
                }
            }
            // consume it
            let _ = reader.next();
        }
        if formal {
            let mut expr = FormalExpression::default();
            let mut body = String::new();
            for e in attributes.drain(..) {
                if let Token::Attribute { local, value, .. } = e {
                    match local.as_str() {
                        "id" => expr.id = Some(value.to_string()),
                        "language" => expr.language = Some(value.to_string()),
                        "evaluatesToTypeRef" => expr.evaluates_totype_ref = Some(value.to_string()),
                        _ => {}
                    }
                }
            }
            let mut done = false;
            while !done {
                if let Some(token) = reader.peek() {
                    match token {
                        Ok(Token::ElementStart { prefix, local, .. })
                            if prefix.as_str() == "bpmn" && local.as_str() == "documentation" =>
                        {
                            expr.documentations
                                .push(Documentation::from_reader(reader)?);
                            continue;
                        }
                        Ok(Token::ElementStart { prefix, local, .. })
                            if prefix.as_str() == "bpmn"
                                && local.as_str() == "extensionElements" =>
                        {
                            expr.extension_elements = Some(ExtensionElements::from_reader(reader)?);
                            continue;
                        }
                        Ok(Token::ElementStart { .. }) => {
                            return Err(XmlError::UnexpectedToken {
                                token: format!("{:?}", token),
                            });
                        }
                        Ok(Token::Text { text, .. }) | Ok(Token::Cdata { text, .. }) => {
                            body.push_str(text.as_str());
                        }
                        Ok(Token::ElementEnd { .. }) => {
                            done = true;
                        }
                        Err(_) => {
                            return Err(XmlError::Parser(reader.next().unwrap().unwrap_err()));
                        }
                        _ => {}
                    }
                } else {
                    return Err(XmlError::UnexpectedEof);
                }
                let _ = reader.next(); // consume it
            }
            if !body.is_empty() {
                expr.content = Some(body);
            }
            Ok(Expr::FormalExpression(expr))
        } else {
            let mut expr = Expression::default();
            for e in attributes.drain(..) {
                if let Token::Attribute { local, value, .. } = e {
                    #[allow(clippy::single_match)] // just to match the style with formal expression
                    match local.as_str() {
                        "id" => expr.id = Some(value.to_string()),
                        _ => {}
                    }
                }
            }
            let mut done = false;
            while !done {
                if let Some(token) = reader.peek() {
                    match token {
                        Ok(Token::ElementStart { prefix, local, .. })
                            if prefix.as_str() == "bpmn" && local.as_str() == "documentation" =>
                        {
                            expr.documentations
                                .push(Documentation::from_reader(reader)?);
                            continue;
                        }
                        Ok(Token::ElementStart { prefix, local, .. })
                            if prefix.as_str() == "bpmn"
                                && local.as_str() == "extensionElements" =>
                        {
                            expr.extension_elements = Some(ExtensionElements::from_reader(reader)?);
                            continue;
                        }
                        Ok(Token::ElementStart { .. }) => {
                            return Err(XmlError::UnexpectedToken {
                                token: format!("{:?}", token),
                            });
                        }
                        Ok(Token::ElementEnd { .. }) => {
                            done = true;
                        }
                        Err(_) => {
                            return Err(XmlError::Parser(reader.next().unwrap().unwrap_err()));
                        }
                        _ => {}
                    }
                } else {
                    return Err(XmlError::UnexpectedEof);
                }

                let _ = reader.next(); // consume it
            }
            Ok(Expr::Expression(expr))
        }
    }
}
