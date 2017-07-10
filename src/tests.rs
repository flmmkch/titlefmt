use super::*;

use std::collections::HashMap;

pub struct MetadataProvider<'a> {
    metadata_dict: HashMap<&'a str, &'a str>,
}

impl<'a> metadata::Provider for MetadataProvider<'a> {
    fn tag_value(&self, key: &str) -> Option<String> {
        let entry = self.metadata_dict.get(key);
        if let Some(value) = entry {
            let s = value.to_string();
            Some(s)
        }
        else {
            None
        }
    }
}

impl<'a> MetadataProvider<'a> {
    pub fn new(metadata_dict: HashMap<&'a str, &'a str>) -> MetadataProvider<'a> {
        MetadataProvider {
            metadata_dict,
        }
    }
}

#[test]
fn test_parser_optional() {
    let formatter = Formatter::new();
    // tests with optional expressions
    {
        let expression = formatter.parser().parse("%tracknumber%. %title%[ (%composer%)]").unwrap();
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
        let expression = formatter.parser().parse("%tracknumber%. %title%[ (%composer%)[ '['%testfield%']'] - hop]").unwrap();
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


#[test]
fn test_function_add()
{
    let formatter = Formatter::new();
    // tests with functions
    {
        let test_metadata = {
            let mut dict = HashMap::new();
            dict.insert("title", "1969");
            dict.insert("date", "2017");
            dict.insert("artist", "Ulver");
            ::tests::MetadataProvider::new(dict)
        };
        {
            let expression = formatter.parser().parse("$add(2,3)").unwrap();
            let s = expression.apply(&test_metadata);
            assert_eq!("5", s.to_string().as_str());
        }
        {
            let expression = formatter.parser().parse("$add(2,3,4,5)").unwrap();
            let s = expression.apply(&test_metadata);
            assert_eq!("14", s.to_string().as_str());
        }
        {
            let expression = formatter.parser().parse("$add(2,3,-4,5)").unwrap();
            let s = expression.apply(&test_metadata);
            assert_eq!("6", s.to_string().as_str());
        }
        {
            let expression = formatter.parser().parse("$if($add(2,3),ok,no)").unwrap();
            let s = expression.apply(&test_metadata);
            assert_eq!("no", s.to_string().as_str());
        }
        {
            let expression = formatter.parser().parse("$add(%date%,1)").unwrap();
            let s = expression.apply(&test_metadata);
            assert_eq!("2018", s.to_string().as_str());
        }
        {
            let expression = formatter.parser().parse("$add(%date%,-1)").unwrap();
            let s = expression.apply(&test_metadata);
            assert_eq!("2016", s.to_string().as_str());
        }
        {
            let expression = formatter.parser().parse("$if($add(%title%,1),ok,no)").unwrap();
            let s = expression.apply(&test_metadata);
            assert_eq!("ok", s.to_string().as_str());
        }
        {
            let expression = formatter.parser().parse("$if($greater($add(%title%,1),1969),ok,no)").unwrap();
            let s = expression.apply(&test_metadata);
            assert_eq!("ok", s.to_string().as_str());
        }
    }
}

#[test]
fn test_function_div()
{
    let formatter = Formatter::new();
    // tests with functions
    {
        let test_metadata = ::tests::MetadataProvider::new(HashMap::new());
        {
            let expression = formatter.parser().parse("$div(7,3)").unwrap();
            let s = expression.apply(&test_metadata);
            assert_eq!("2", s.to_string().as_str());
        }
    }
}

#[test]
fn test_function_greater()
{
    let formatter = Formatter::new();
    // tests with functions
    {
        let test_metadata = ::tests::MetadataProvider::new(HashMap::new());
        {
            let expression = formatter.parser().parse("$if($greater(7,3), ok, no)").unwrap();
            let s = expression.apply(&test_metadata);
            assert_eq!("ok", s.to_string().as_str());
        }
        {
            let expression = formatter.parser().parse("$if($greater(1,3), ok, no)").unwrap();
            let s = expression.apply(&test_metadata);
            assert_eq!("no", s.to_string().as_str());
        }
    }
}


#[test]
fn test_function_max()
{
    let formatter = Formatter::new();
    // tests with functions
    {
        let test_metadata = ::tests::MetadataProvider::new(HashMap::new());
        {
            let expression = formatter.parser().parse("$max(7,3)").unwrap();
            let s = expression.apply(&test_metadata);
            assert_eq!("7", s.to_string().as_str());
        }
        {
            let expression = formatter.parser().parse("$max(3,7)").unwrap();
            let s = expression.apply(&test_metadata);
            assert_eq!("7", s.to_string().as_str());
        }
        {
            let expression = formatter.parser().parse("$max(7,3,8,5)").unwrap();
            let s = expression.apply(&test_metadata);
            assert_eq!("8", s.to_string().as_str());
        }
    }
}

#[test]
fn test_function_min()
{
    let formatter = Formatter::new();
    // tests with functions
    {
        let test_metadata = ::tests::MetadataProvider::new(HashMap::new());
        {
            let expression = formatter.parser().parse("$min(7,3)").unwrap();
            let s = expression.apply(&test_metadata);
            assert_eq!("3", s.to_string().as_str());
        }
        {
            let expression = formatter.parser().parse("$min(3,7)").unwrap();
            let s = expression.apply(&test_metadata);
            assert_eq!("3", s.to_string().as_str());
        }
        {
            let expression = formatter.parser().parse("$min(7,3,2,5)").unwrap();
            let s = expression.apply(&test_metadata);
            assert_eq!("2", s.to_string().as_str());
        }
    }
}

#[test]
fn test_function_mod()
{
    let formatter = Formatter::new();
    // tests with functions
    {
        let test_metadata = ::tests::MetadataProvider::new(HashMap::new());
        {
            let expression = formatter.parser().parse("$mod(7,3)").unwrap();
            let s = expression.apply(&test_metadata);
            assert_eq!("1", s.to_string().as_str());
        }
        {
            let expression = formatter.parser().parse("$mod(15,6)").unwrap();
            let s = expression.apply(&test_metadata);
            assert_eq!("3", s.to_string().as_str());
        }
    }
}

#[test]
fn test_function_xor()
{
    let formatter = Formatter::new();
    // tests with functions
    {
        let test_metadata = {
            let mut dict = HashMap::new();
            dict.insert("title", "Flood");
            dict.insert("artist", "Boris");
            ::tests::MetadataProvider::new(dict)
        };
        {
            let expression = formatter.parser().parse("$if($xor(test, test2, test3), ok, not)").unwrap();
            let s = expression.apply(&test_metadata);
            assert_eq!("not", s.to_string().as_str());
        }
        {
            let expression = formatter.parser().parse("$if($xor(%title%, test2, test3), ok, not)").unwrap();
            let s = expression.apply(&test_metadata);
            assert_eq!("ok", s.to_string().as_str());
        }
        {
            let expression = formatter.parser().parse("$if($xor(%title%, %artist%, test3), ok, not)").unwrap();
            let s = expression.apply(&test_metadata);
            assert_eq!("not", s.to_string().as_str());
        }
        {
            let expression = formatter.parser().parse("$if($xor(%title%, %artist%, %title%. test), ok, not)").unwrap();
            let s = expression.apply(&test_metadata);
            assert_eq!("ok", s.to_string().as_str());
        }
        {
            let expression = formatter.parser().parse("$if($xor(%title%, %album%, test3), ok, not)").unwrap();
            let s = expression.apply(&test_metadata);
            assert_eq!("ok", s.to_string().as_str());
        }
    }
}

#[test]
fn test_function_if()
{
    let formatter = Formatter::new();
    // tests with functions
    {
        let test_metadata = {
            let mut dict = HashMap::new();
            dict.insert("tracknumber", "9");
            dict.insert("title", "9th Symphony");
            dict.insert("composer", "Beethoven");
            ::tests::MetadataProvider::new(dict)
        };
        {
            let expression = formatter.parser().parse("$if(test, ok, non)").unwrap();
            let s = expression.apply(&test_metadata);
            assert_eq!("non", s.to_string().as_str());
        }
        {
            let expression = formatter.parser().parse("$if(%title%, ok, non)").unwrap();
            let s = expression.apply(&test_metadata);
            assert_eq!("ok", s.to_string().as_str());
        }
        {
            let expression = formatter.parser().parse("$if(%artist%, ok, non)").unwrap();
            let s = expression.apply(&test_metadata);
            assert_eq!("non", s.to_string().as_str());
        }
    }
}

#[test]
fn test_function_if2()
{
    let formatter = Formatter::new();
    // tests with functions
    {
        let test_metadata = {
            let mut dict = HashMap::new();
            dict.insert("tracknumber", "9");
            dict.insert("title", "9th Symphony");
            dict.insert("composer", "Beethoven");
            ::tests::MetadataProvider::new(dict)
        };
        {
            let expression = formatter.parser().parse("%tracknumber%. $if2(%composer%, %tracknumber%) - %title%").unwrap();
            let s = expression.apply(&test_metadata);
            assert_eq!("09. Beethoven - 9th Symphony", s.to_string().as_str());
        }
        {
            let expression = formatter.parser().parse("%tracknumber%. $if2(%artist%, %composer%) - %title%").unwrap();
            let s = expression.apply(&test_metadata);
            assert_eq!("09. Beethoven - 9th Symphony", s.to_string().as_str());
        }
        {
            let expression = formatter.parser().parse("%tracknumber%. $if2(%composer%, %artist%) - %title%").unwrap();
            let s = expression.apply(&test_metadata);
            assert_eq!("09. Beethoven - 9th Symphony", s.to_string().as_str());
        }
        {
            let expression = formatter.parser().parse("%tracknumber%. $if2(%albumartist%, %artist%) - %title%").unwrap();
            let s = expression.apply(&test_metadata);
            assert_eq!("09. ? - 9th Symphony", s.to_string().as_str());
        }
    }
}

#[test]
fn test_function_if3()
{
    let formatter = Formatter::new();
    // tests with functions
    {
        let test_metadata = {
            let mut dict = HashMap::new();
            dict.insert("title", "1969");
            dict.insert("date", "2017");
            dict.insert("artist", "Ulver");
            ::tests::MetadataProvider::new(dict)
        };
        {
            let expression = formatter.parser().parse("$if3(%composer%, %tracknumber%, %title%)").unwrap();
            let s = expression.apply(&test_metadata);
            assert_eq!("1969", s.to_string().as_str());
        }
        {
            let expression = formatter.parser().parse("$if3(%composer%, %title%, %tracknumber%)").unwrap();
            let s = expression.apply(&test_metadata);
            assert_eq!("1969", s.to_string().as_str());
        }
    }
}

#[test]
fn test_function_select()
{
    let formatter = Formatter::new();
    // tests with functions
    {
        let test_metadata = ::tests::MetadataProvider::new(HashMap::new());
        {
            let expression = formatter.parser().parse("$select(2, one, two, three, four)").unwrap();
            let s = expression.apply(&test_metadata);
            assert_eq!("two", s.to_string().as_str());
        }
        {
            let expression = formatter.parser().parse("$select(4, one, two, three, four)").unwrap();
            let s = expression.apply(&test_metadata);
            assert_eq!("four", s.to_string().as_str());
        }
        {
            let expression = formatter.parser().parse("$select(5, one, two, three, four)").unwrap();
            let s = expression.apply(&test_metadata);
            assert_eq!("", s.to_string().as_str());
        }
        {
            let expression = formatter.parser().parse("$select(6, one, two, three, four)").unwrap();
            let s = expression.apply(&test_metadata);
            assert_eq!("", s.to_string().as_str());
        }
        {
            let expression = formatter.parser().parse("$select(-1, one, two, three, four)").unwrap();
            let s = expression.apply(&test_metadata);
            assert_eq!("", s.to_string().as_str());
        }
    }
}

#[test]
fn test_string_functions()
{
    let formatter = Formatter::new();
    // tests with functions
    {
        let test_metadata = ::tests::MetadataProvider::new(HashMap::new());
        {
            let expression = formatter.parser().parse("$abbr(hello world)").unwrap();
            let s = expression.apply(&test_metadata);
            assert_eq!("hw", s.to_string().as_str());
        }
        {
            let expression = formatter.parser().parse("$abbr('Hunting & Gathering (Cydonia)')").unwrap();
            let s = expression.apply(&test_metadata);
            assert_eq!("H&GC", s.to_string().as_str());
        }
        {
            let expression = formatter.parser().parse("$abbr('21st Century Schizoid Man (including Mirrors)')").unwrap();
            let s = expression.apply(&test_metadata);
            assert_eq!("2CSMiM", s.to_string().as_str());
        }
        {
            let expression = formatter.parser().parse("$caps(hello world)").unwrap();
            let s = expression.apply(&test_metadata);
            assert_eq!("Hello World", s.to_string().as_str());
        }
        {
            let expression = formatter.parser().parse("$caps(ça t''étonne?)").unwrap();
            let s = expression.apply(&test_metadata);
            assert_eq!("Ça T'étonne?", s.to_string().as_str());
        }
        {
            let expression = formatter.parser().parse("$caps(ÇA T''ÉTONNE ね?)").unwrap();
            let s = expression.apply(&test_metadata);
            assert_eq!("Ça T'étonne ね?", s.to_string().as_str());
        }
        {
            let expression = formatter.parser().parse("$cut(hello, 1)").unwrap();
            let s = expression.apply(&test_metadata);
            assert_eq!("h", s.to_string().as_str());
        }
        {
            let expression = formatter.parser().parse("$cut(小さな恋のうた, 3)").unwrap();
            let s = expression.apply(&test_metadata);
            assert_eq!("小さな", s.to_string().as_str());
        }
    }
}
