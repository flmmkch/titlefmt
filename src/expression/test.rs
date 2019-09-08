// Parsing tests
use super::{Expression, Item};
use crate::test::MetadataProvider;
use crate::Formatter;
use std::collections::HashMap;

fn make_item_text(text: &str) -> Item<MetadataProvider> {
    Item::Text(text.to_owned())
}

fn make_item_tag(tag: &str) -> Item<MetadataProvider> {
    Item::Tag(tag.to_owned())
}

#[test]
fn test_apply_simple_text() {
    let expression = {
        let mut items = Vec::new();
        items.push(make_item_text("test"));
        items.push(make_item_text("hello world"));
        Expression::new(items)
    };
    let test_metadata = {
        let mut dict = HashMap::new();
        dict.insert("title", "Test Song");
        MetadataProvider::new(dict)
    };
    let s = expression.apply(&test_metadata);
    assert_eq!("testhello world", s.to_string().as_str());
}

#[test]
fn test_apply_tags() {
    {
        let expression = {
            let mut items = Vec::new();
            items.push(make_item_text("test "));
            items.push(make_item_tag("title"));
            items.push(make_item_text(" hello world"));
            Expression::new(items)
        };
        let test_metadata = {
            let mut dict = HashMap::new();
            dict.insert("title", "Test Song");
            MetadataProvider::new(dict)
        };
        let s = expression.apply(&test_metadata);
        assert_eq!("test Test Song hello world", s.to_string().as_str());
    }
    {
        let expression = {
            let mut items = Vec::new();
            items.push(make_item_tag("tracknumber"));
            items.push(make_item_text(". "));
            items.push(make_item_tag("artist"));
            items.push(make_item_text(" - "));
            items.push(make_item_tag("title"));
            Expression::new(items)
        };
        {
            let test_metadata = {
                let mut dict = HashMap::new();
                dict.insert("tracknumber", "01");
                dict.insert("artist", "NewArtist");
                dict.insert("title", "Test Song");
                MetadataProvider::new(dict)
            };
            let s = expression.apply(&test_metadata);
            assert_eq!("01. NewArtist - Test Song", s.to_string().as_str());
        }
        {
            let test_metadata = {
                let mut dict = HashMap::new();
                dict.insert("tracknumber", "01");
                dict.insert("title", "Test Song");
                MetadataProvider::new(dict)
            };
            let s = expression.apply(&test_metadata);
            assert_eq!("01. ? - Test Song", s.to_string().as_str());
        }
    }
}

#[test]
fn test_apply_optional() {
    let expression = {
        let mut items = Vec::new();
        items.push(make_item_tag("tracknumber"));
        items.push(make_item_text(". "));
        items.push(make_item_tag("title"));
        {
            let mut sub_items = Vec::new();
            sub_items.push(make_item_text(" ("));
            sub_items.push(make_item_tag("composer"));
            sub_items.push(make_item_text(")"));
            let sub_expr = Expression::new(sub_items);
            items.push(Item::OptionalExpr(Box::new(sub_expr)));
        }
        Expression::new(items)
    };
    {
        let test_metadata = {
            let mut dict = HashMap::new();
            dict.insert("tracknumber", "9");
            dict.insert("title", "9th Symphony");
            dict.insert("composer", "Beethoven");
            MetadataProvider::new(dict)
        };
        let s = expression.apply(&test_metadata);
        assert_eq!("09. 9th Symphony (Beethoven)", s.to_string().as_str());
    }
    {
        let test_metadata = {
            let mut dict = HashMap::new();
            dict.insert("tracknumber", "5");
            dict.insert("title", "Greensleeves");
            MetadataProvider::new(dict)
        };
        let s = expression.apply(&test_metadata);
        assert_eq!("05. Greensleeves", s.to_string().as_str());
    }
}

#[test]
fn test_parsed() {
    let formatter = Formatter::new();
    // tests with optional expressions
    {
        let expression = formatter
            .parser()
            .parse("%tracknumber%. %title%[ (%composer%)]")
            .unwrap();
        {
            let test_metadata = {
                let mut dict = HashMap::new();
                dict.insert("tracknumber", "9");
                dict.insert("title", "9th Symphony");
                dict.insert("composer", "Beethoven");
                MetadataProvider::new(dict)
            };
            let s = expression.apply(&test_metadata);
            assert_eq!("09. 9th Symphony (Beethoven)", s.to_string().as_str());
        }
        {
            let test_metadata = {
                let mut dict = HashMap::new();
                dict.insert("tracknumber", "5");
                dict.insert("title", "Greensleeves");
                MetadataProvider::new(dict)
            };
            let s = expression.apply(&test_metadata);
            assert_eq!("05. Greensleeves", s.to_string().as_str());
        }
    }
    {
        let expression = formatter
            .parser()
            .parse("%tracknumber%. %title%[ (%composer%)[ '['%testfield%']'] - hop]")
            .unwrap();
        {
            let test_metadata = {
                let mut dict = HashMap::new();
                dict.insert("tracknumber", "9");
                dict.insert("title", "9th Symphony");
                dict.insert("composer", "Beethoven");
                MetadataProvider::new(dict)
            };
            let s = expression.apply(&test_metadata);
            assert_eq!("09. 9th Symphony (Beethoven) - hop", s.to_string().as_str());
        }
        {
            let test_metadata = {
                let mut dict = HashMap::new();
                dict.insert("tracknumber", "5");
                dict.insert("title", "Greensleeves");
                dict.insert("testfield", "OK");
                MetadataProvider::new(dict)
            };
            let s = expression.apply(&test_metadata);
            assert_eq!("05. Greensleeves (?) [OK] - hop", s.to_string().as_str());
        }
        {
            let test_metadata = {
                let mut dict = HashMap::new();
                dict.insert("tracknumber", "5");
                dict.insert("title", "Greensleeves");
                MetadataProvider::new(dict)
            };
            let s = expression.apply(&test_metadata);
            assert_eq!("05. Greensleeves", s.to_string().as_str());
        }
    }
}
