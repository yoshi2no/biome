use crate::formatter_traits::FormatTokenAndNode;

use crate::{
    format_elements, space_token, FormatElement, FormatResult, Formatter, ToFormatElement,
};

use rslint_parser::ast::JsElseClause;
use rslint_parser::ast::JsElseClauseFields;

impl ToFormatElement for JsElseClause {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsElseClauseFields {
            else_token,
            alternate,
        } = self.as_fields();

        Ok(format_elements![
            else_token.format(formatter)?,
            space_token(),
            alternate.format(formatter)?,
        ])
    }
}