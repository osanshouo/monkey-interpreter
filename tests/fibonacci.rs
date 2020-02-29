use monkey_interpreter::{
    evaluate,
    object::Object,
    error::MonkeyError,
};

#[test]
fn fibonacci() -> Result<(), MonkeyError> {
    let answers = [ 1, 1, 2, 3, 5, 8, 13, 21 ];

    for (n, answer) in answers.iter().enumerate() {
        let input = INPUT.replace("NUMBER", &n.to_string());
        assert_eq!( evaluate(&input)?, Object::Integer(*answer) );
    }
    
    Ok(())
}

const INPUT: &str = r#"
let phi = fn(n) {
    if (n == 0) { return 1; }
    if (n == 1) { return 1; }
    phi(n-1) + phi(n-2);
};

phi(NUMBER);
"#;
