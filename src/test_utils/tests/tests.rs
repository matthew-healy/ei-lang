use test_utils::test_with_parameters;

#[test_with_parameters(
    [ input ]
    [ 1     ]
    [ 3     ]
    [ 5     ]
)]
fn single_param(input: i64) {
    assert_eq!(0, input - input);
}

#[test_with_parameters(
    [ input, expected ]
    [ 1    , "1"      ]
    [ 25   , "25"     ]
    [ 900  , "900"    ]
)]
fn input_output_param(input: i32, expected: &str) {
    let result = input.to_string();
    assert_eq!(result, expected);
}

struct Person<'a> {
    first_name: &'a str,
    last_name: &'a str,
}

impl <'a> Person<'a> {
    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name) 
    }
}

#[test_with_parameters(
    [ first_name, last_name, expected        ]
    [ "Matthew" , "Healy"  , "Matthew Healy" ]
    [ "Karima"  , "Walker" , "Karima Walker" ]
)]
fn full_name_with_two_input_params(
    first_name: &str,
    last_name: &str,
    expected: &str
) {
    let person = Person { first_name, last_name };
    assert_eq!(expected, person.full_name())
}

#[test_with_parameters(
    [ person                                              , full_name       ]
    [ Person { first_name: "Matthew", last_name: "Healy" }, "Matthew Healy" ]
    [ Person { first_name: "Joanna", last_name: "Newsom" }, "Joanna Newsom" ]
)]
fn full_name_with_person_struct(person: Person, full_name: &str) {
    assert_eq!(full_name, person.full_name())
}