extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;

#[derive(Parser)]
#[grammar = "../src/pest_grammar/scheme.pest"]
pub struct SchemeParser;

#[test]
fn test_integer_type_name() {
    assert!(SchemeParser::parse(Rule::integer_type_name, "Integer").is_ok());
    assert!(SchemeParser::parse(Rule::integer_type_name, "Int").is_err());
}

#[test]
fn test_scalar_type() {
    assert!(SchemeParser::parse(Rule::scalar_type, "Integer").is_ok());
    assert!(SchemeParser::parse(Rule::scalar_type, "Char").is_ok());
    assert!(SchemeParser::parse(Rule::scalar_type, "Int").is_err());
}

#[test]
fn test_type_constructor_name() {
    assert!(
        SchemeParser::parse(Rule::type_constructor_name, "Integer").is_err(),
        "Reserved as a primitive names"
    );

    assert!(
        SchemeParser::parse(Rule::type_constructor_name, "myType").is_err(),
        "Must be UpperCamelCase"
    );
    assert!(
        SchemeParser::parse(Rule::type_constructor_name, "my_type").is_err(),
        "Must be UpperCamelCase"
    );
    assert_eq!(
        SchemeParser::parse(Rule::type_constructor_name, "MY_TYPE")
            .unwrap()
            .next()
            .unwrap()
            .into_inner()
            .as_str(),
        "MY",
        "`_` is not allowed"
    );
    assert_eq!(
        SchemeParser::parse(Rule::type_constructor_name, "My Type")
            .unwrap()
            .next()
            .unwrap()
            .into_inner()
            .as_str(),
        "My",
        "` ` is not allowed"
    );

    assert_eq!(
        SchemeParser::parse(Rule::type_constructor_name, "MyType")
            .unwrap()
            .next()
            .unwrap()
            .into_inner()
            .as_str(),
        "MyType",
    );
    assert_eq!(
        SchemeParser::parse(Rule::type_constructor_name, "MYTYPE")
            .unwrap()
            .next()
            .unwrap()
            .into_inner()
            .as_str(),
        "MYTYPE",
    );
    assert_eq!(
        SchemeParser::parse(Rule::type_constructor_name, "My1Type2")
            .unwrap()
            .next()
            .unwrap()
            .into_inner()
            .as_str(),
        "My1Type2",
    );
}

#[test]
fn test_product_type() {
    assert!(
        SchemeParser::parse(Rule::product_type, "Integer()").is_err(),
        "Reserved as a primitive names"
    );

    assert_eq!(
        SchemeParser::parse(Rule::product_type, "MyProduct()")
            .unwrap()
            .next()
            .unwrap()
            .into_inner()
            .as_str(),
        "MyProduct()",
    );
    assert_eq!(
        SchemeParser::parse(Rule::product_type, "MyProduct  ()")
            .unwrap()
            .next()
            .unwrap()
            .into_inner()
            .as_str(),
        "MyProduct  ()",
    );
    assert_eq!(
        SchemeParser::parse(Rule::product_type, "MyProduct(Integer)")
            .unwrap()
            .next()
            .unwrap()
            .into_inner()
            .as_str(),
        "MyProduct(Integer)",
    );
    assert_eq!(
        SchemeParser::parse(Rule::product_type, "MyProduct(Integer, Char)")
            .unwrap()
            .next()
            .unwrap()
            .into_inner()
            .as_str(),
        "MyProduct(Integer, Char)",
    );
    assert_eq!(
        SchemeParser::parse(Rule::product_type, "MyProduct(Integer, Char, Set[Integer])")
            .unwrap()
            .next()
            .unwrap()
            .into_inner()
            .as_str(),
        "MyProduct(Integer, Char, Set[Integer])",
    );
}

#[test]
fn test_union_type() {
    assert!(
        SchemeParser::parse(Rule::union_type, "Integer () | (Char)").is_err(),
        "Reserved as a primitive names"
    );

    assert!(
        SchemeParser::parse(Rule::union_type, "MyUnion() | Integer").is_err(),
        "White space required after type constructor name."
    );

    assert_eq!(
        SchemeParser::parse(Rule::union_type, "UNION ")
            .unwrap()
            .next()
            .unwrap()
            .into_inner()
            .as_str(),
        "MyUnion () | ()",
        "Allowed grammatically."
    );
}
