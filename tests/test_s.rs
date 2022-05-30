use string_literal::s;

#[derive(Debug, PartialEq)]
struct SomeStruct {
    string_field: String,
    other_string_field: String,
    int_field: isize,
    other_struct: OtherStruct,
}
#[derive(Debug, PartialEq)]
struct OtherStruct {
    a: String,
    //TODO: make embed macro work
   // b: Vec<String>,
}

#[test]
fn test_s() {
    let want = SomeStruct {
        string_field: "string".to_string(),
        other_string_field: "other string".to_string(),
        int_field: 3,
        other_struct: OtherStruct {
            a: "a".to_string(),
            //b: vec!["1".to_string(), "2".to_string()],
        },
    };
    let got = s! {
        SomeStruct {
            string_field: "string",
            other_string_field: "other string",
            int_field: 3,
            other_struct: OtherStruct {
                a: "a",
                //b: vec!["1", "2"],
            },
        }
    };
    assert_eq!(want, got);
}
