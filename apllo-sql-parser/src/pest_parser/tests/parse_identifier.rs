use super::super::{PestParser, PestResult, Rule};
use pest::Parser;

struct AcceptedTestParameter<'a>(&'a str);
macro_rules! accepted_parameterized_tests {
    ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() -> PestResult<()> {
                let param: AcceptedTestParameter = $value;

                let mut parse_result = PestParser::parse(Rule::identifier, param.0)?;
                if let Some(identifier_pair) = parse_result.next() {
                    assert_eq!(identifier_pair.as_rule(), Rule::identifier);
                    assert_eq!(identifier_pair.as_str(), param.0);
                } else {
                    panic!("'{}' is expected to be parsed as an identifier.", param.0);
                }

                Ok(())
            }
        )*
    }
}

struct PartiallyAcceptedTestParameter<'a> {
    input: &'a str,
    accepted: &'a str,
}
macro_rules! partially_accepted_parameterized_tests {
    ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() -> PestResult<()> {
                let param: PartiallyAcceptedTestParameter = $value;
                assert!(param.input.starts_with(param.accepted));
                assert_ne!(param.input, param.accepted);

                let mut parse_result = PestParser::parse(Rule::identifier, param.input)?;
                if let Some(identifier_pair) = parse_result.next() {
                    assert_eq!(identifier_pair.as_rule(), Rule::identifier);
                    assert_eq!(identifier_pair.as_str(), param.accepted);
                } else {
                    panic!("'{}' is expected to be parsed as an identifier: '{}'.", param.input, param.accepted);
                }

                Ok(())
            }
        )*
    }
}

struct RejectedTestParameter<'a>(&'a str);
macro_rules! rejected_parameterized_tests {
    ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let param: RejectedTestParameter = $value;
                assert!(PestParser::parse(Rule::identifier, param.0).is_err());
            }
        )*
    }
}

accepted_parameterized_tests! {
    single_character: AcceptedTestParameter("a"),

    snake_case: AcceptedTestParameter("abc_def"),
    lower_camel_case: AcceptedTestParameter("abcDef"),
    upper_camel_case: AcceptedTestParameter("AbcDef"),

    with_digits: AcceptedTestParameter("a1"),
    with_full_width_digits: AcceptedTestParameter("a１"),

    hiragana: AcceptedTestParameter("あいうえお"),
    katakana: AcceptedTestParameter("アイウエオ"),
    kanji: AcceptedTestParameter("一二三"),

    emoji: AcceptedTestParameter("🥺🥰😡🍣🍺"),

    has_keyword1: AcceptedTestParameter("SELECT_"),
    has_keyword2: AcceptedTestParameter("_SELECT"),
}

partially_accepted_parameterized_tests! {
    has_space: PartiallyAcceptedTestParameter { input: "a b", accepted: "a" },
    has_full_width_space: PartiallyAcceptedTestParameter { input: "a　b", accepted: "a" },

    has_ascii_33: PartiallyAcceptedTestParameter { input: "a!", accepted: "a" },
    has_ascii_34: PartiallyAcceptedTestParameter { input: "a\"", accepted: "a" },
    has_ascii_35: PartiallyAcceptedTestParameter { input: "a#", accepted: "a" },
    has_ascii_36: PartiallyAcceptedTestParameter { input: "a$", accepted: "a" },
    has_ascii_37: PartiallyAcceptedTestParameter { input: "a%", accepted: "a" },
    has_ascii_38: PartiallyAcceptedTestParameter { input: "a&", accepted: "a" },
    has_ascii_39: PartiallyAcceptedTestParameter { input: "a'", accepted: "a" },
    has_ascii_40: PartiallyAcceptedTestParameter { input: "a(", accepted: "a" },
    has_ascii_41: PartiallyAcceptedTestParameter { input: "a)", accepted: "a" },
    has_ascii_42: PartiallyAcceptedTestParameter { input: "a*", accepted: "a" },
    has_ascii_43: PartiallyAcceptedTestParameter { input: "a+", accepted: "a" },
    has_ascii_44: PartiallyAcceptedTestParameter { input: "a,", accepted: "a" },
    has_ascii_45: PartiallyAcceptedTestParameter { input: "a-", accepted: "a" },
    has_ascii_46: PartiallyAcceptedTestParameter { input: "a.", accepted: "a" },
    has_ascii_47: PartiallyAcceptedTestParameter { input: "a/", accepted: "a" },
    has_ascii_58: PartiallyAcceptedTestParameter { input: "a:", accepted: "a" },
    has_ascii_59: PartiallyAcceptedTestParameter { input: "a;", accepted: "a" },
    has_ascii_60: PartiallyAcceptedTestParameter { input: "a<", accepted: "a" },
    has_ascii_61: PartiallyAcceptedTestParameter { input: "a=", accepted: "a" },
    has_ascii_62: PartiallyAcceptedTestParameter { input: "a>", accepted: "a" },
    has_ascii_63: PartiallyAcceptedTestParameter { input: "a?", accepted: "a" },
    has_ascii_64: PartiallyAcceptedTestParameter { input: "a@", accepted: "a" },
    has_ascii_91: PartiallyAcceptedTestParameter { input: "a[", accepted: "a" },
    has_ascii_92: PartiallyAcceptedTestParameter { input: "a\\", accepted: "a" },
    has_ascii_93: PartiallyAcceptedTestParameter { input: "a]", accepted: "a" },
    has_ascii_94: PartiallyAcceptedTestParameter { input: "a^", accepted: "a" },
    has_ascii_96: PartiallyAcceptedTestParameter { input: "a`", accepted: "a" },
    has_ascii_123: PartiallyAcceptedTestParameter { input: "a{", accepted: "a" },
    has_ascii_124: PartiallyAcceptedTestParameter { input: "a|", accepted: "a" },
    has_ascii_125: PartiallyAcceptedTestParameter { input: "a}", accepted: "a" },
    has_ascii_126: PartiallyAcceptedTestParameter { input: "a~", accepted: "a" },
}

rejected_parameterized_tests! {
    starts_with_digit: RejectedTestParameter("1a"),
    starts_with_full_width_digit: RejectedTestParameter("１a"),

    starts_with_plus_sign: RejectedTestParameter("+"),
    starts_with_minus_sign: RejectedTestParameter("-"),

    keyword: RejectedTestParameter("SELECT"),
}
