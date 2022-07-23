use crate::evaluator::eval;
use crate::lexer::Lexer;
use crate::object::environment::Environment;
use crate::object::{Object, ObjectInterface, ObjectType};
use crate::parser::Parser;
use std::any::{Any, TypeId};
use crate::object::string::StringObj;

fn test_eval_integer_expression() -> anyhow::Result<()> {
    struct Test {
        input: String,
        expected: i64,
    }

    let tests = vec![
        Test {
            input: "5".into(),
            expected: 5,
        },
        Test {
            input: "10".into(),
            expected: 10,
        },
        Test {
            input: "-5".into(),
            expected: -5,
        },
        Test {
            input: "-10".into(),
            expected: -10,
        },
        Test {
            input: "5 + 5 + 5 + 5 - 10".into(),
            expected: 10,
        },
        Test {
            input: "2 * 2 * 2 * 2 * 2".into(),
            expected: 32,
        },
        Test {
            input: "-50 + 100 + -50".into(),
            expected: 0,
        },
        Test {
            input: "5 * 2 + 10".into(),
            expected: 20,
        },
        Test {
            input: "5 + 2 * 10".into(),
            expected: 25,
        },
        Test {
            input: "20 + 2 * -10".into(),
            expected: 0,
        },
        Test {
            input: "50 / 2 * 2 + 10".into(),
            expected: 60,
        },
        Test {
            input: "2 * (5 + 10)".into(),
            expected: 30,
        },
        Test {
            input: "3 * 3 * 3 + 10".into(),
            expected: 37,
        },
        Test {
            input: "3 * (3 * 3) + 10".into(),
            expected: 37,
        },
        Test {
            input: "(5 + 10 * 2 + 15 /3) * 2 + -10".into(),
            expected: 50,
        },
    ];

    for tt in tests {
        let evaluated = test_eval(tt.input)?;

        test_integer_object(evaluated, tt.expected)?;
    }

    Ok(())
}

fn test_eval(input: String) -> anyhow::Result<Object> {
    let lexer = Lexer::new(input.as_str())?;

    let mut parser = Parser::new(lexer)?;

    let program = parser.parse_program()?;

    let mut env = Environment::new();

    Ok(eval(Box::new(program), &mut env)?)
}

fn test_integer_object(obj: Object, expected: i64) -> anyhow::Result<bool> {
    match obj {
        Object::Integer(value) => {
            if value.value != expected {
                eprintln!(
                    "object has wrong value. got = {:?}, want = {:?}",
                    value.value, expected
                );
                return Ok(false);
            }

            Ok(true)
        }
        _ => {
            eprintln!("test_integer_object unimplemented: {:#?}", obj);
            unimplemented!()
        }
    }
}

fn test_eval_boolean_expression() -> anyhow::Result<()> {
    struct Test {
        input: String,
        expected: bool,
    }

    let tests = vec![
        Test {
            input: "true".into(),
            expected: true,
        },
        Test {
            input: "false".into(),
            expected: false,
        },
        Test {
            input: "1 < 2".into(),
            expected: true,
        },
        Test {
            input: "1 > 2".into(),
            expected: false,
        },
        Test {
            input: "1 < 1".into(),
            expected: false,
        },
        Test {
            input: "1 > 1".into(),
            expected: false,
        },
        Test {
            input: "1 == 1".into(),
            expected: true,
        },
        Test {
            input: "1 != 1".into(),
            expected: false,
        },
        Test {
            input: "1 == 2".into(),
            expected: false,
        },
        Test {
            input: "1 != 2".into(),
            expected: true,
        },
        Test {
            input: "true == true".into(),
            expected: true,
        },
        Test {
            input: "false == false".into(),
            expected: true,
        },
        Test {
            input: "true == false".into(),
            expected: false,
        },
        Test {
            input: "true != false".into(),
            expected: true,
        },
        Test {
            input: "false != true".into(),
            expected: true,
        },
        Test {
            input: "(1 < 2) == true".into(),
            expected: true,
        },
        Test {
            input: "(1 < 2) == false".into(),
            expected: false,
        },
        Test {
            input: "(1 > 2) == true".into(),
            expected: false,
        },
        Test {
            input: "(1 > 2) == false".into(),
            expected: true,
        },
    ];

    for tt in tests.iter() {
        let evaluated = test_eval(tt.input.clone())?;

        test_boolean_object(evaluated, tt.expected)?;
    }

    Ok(())
}

fn test_boolean_object(obj: Object, expected: bool) -> anyhow::Result<bool> {
    match obj {
        Object::Boolean(value) => {
            if value.value != expected {
                eprintln!(
                    "object has wrong value. got = {:?}, want = {:?}",
                    value.value, expected
                );
                return Ok(false);
            }

            Ok(true)
        }
        _ => unimplemented!(),
    }
}

fn test_bang_operator() -> anyhow::Result<()> {
    struct Test {
        input: String,
        expected: bool,
    }

    let tests = vec![
        Test {
            input: "!true".into(),
            expected: false,
        },
        Test {
            input: "!false".into(),
            expected: true,
        },
        Test {
            input: "!5".into(),
            expected: false,
        },
        Test {
            input: "!!true".into(),
            expected: true,
        },
        Test {
            input: "!!false".into(),
            expected: false,
        },
        Test {
            input: "!!5".into(),
            expected: true,
        },
    ];

    for tt in tests {
        let evaluated = test_eval(tt.input)?;

        test_boolean_object(evaluated, tt.expected)?;
    }

    Ok(())
}

fn test_if_else_expressions() -> anyhow::Result<()> {
    struct Test {
        input: String,
        expected: Box<dyn Interface>,
    }

    let tests = vec![
        Test {
            input: "if (true) { 10 }".to_string(),
            expected: Box::new(10),
        },
        Test {
            input: "if (false) { 10 }".to_string(),
            expected: Box::new(()),
        },
        Test {
            input: "if (1) { 10 }".to_string(),
            expected: Box::new(10),
        },
        Test {
            input: "if (1 < 2) { 10 }".to_string(),
            expected: Box::new(10),
        },
        Test {
            input: "if (1 > 2) { 10 }".to_string(),
            expected: Box::new(()),
        },
        Test {
            input: "if (1 > 2) { 10 } else { 20 }".to_string(),
            expected: Box::new(20),
        },
        Test {
            input: "if (1 < 2) { 10 } else { 20 }".to_string(),
            expected: Box::new(10),
        },
    ];

    for tt in tests.into_iter() {
        let evaluated = test_eval(tt.input)?;
        let t = tt.expected.as_any().type_id();

        if TypeId::of::<i64>() == t {
            let integer = tt
                .expected
                .as_any()
                .downcast_ref::<i64>()
                .ok_or(anyhow::anyhow!("tt.expected error"))?;

            let ret = test_integer_object(evaluated, integer.clone())?;
            if !ret {
                eprintln!("test integer object error")
            }
        } else if TypeId::of::<()>() == t {
            let ret = test_null_object(evaluated)?;
            if !ret {
                eprintln!("test null object error");
            }
        }
    }

    Ok(())
}

fn test_null_object(obj: Object) -> anyhow::Result<bool> {
    let ret = obj.inspect();
    println!("parser object is {}", ret);
    Ok(true)
}

fn test_return_statements() -> anyhow::Result<()> {
    #[derive(Debug)]
    struct Test {
        input: String,
        expected: i64,
    }

    let tests = vec![
        Test {
            input: "return 10;".to_string(),
            expected: 10,
        },
        Test {
            input: "return 10; 9;".to_string(),
            expected: 10,
        },
        Test {
            input: "return 2 * 5; 9;".to_string(),
            expected: 10,
        },
        Test {
            input: "9; return 2 * 5; 9;".to_string(),
            expected: 10,
        },
        Test {
            input: r#"
if (10 > 1) {
    if (10 > 1) {
        return 10;
    }
    return 1;
}"#
            .to_string(),
            expected: 10,
        },
    ];

    for tt in tests.into_iter() {
        println!("test_return_statements = {:?}", tt);
        let evaluated = test_eval(tt.input)?;

        let ret = test_integer_object(evaluated, tt.expected)?;
        if !ret {
            eprintln!("test return statement failed");
        }
    }

    Ok(())
}

fn test_error_handling() -> anyhow::Result<()> {
    struct Test {
        input: String,
        expected_message: String,
    }

    let tests = vec![
        Test {
            input: "5 + true;".to_string(),
            expected_message: "type mismatch: INTEGER + BOOLEAN".to_string(),
        },
        Test {
            input: "5 + true; 5;".to_string(),
            expected_message: "type mismatch: INTEGER + BOOLEAN".to_string(),
        },
        Test {
            input: "-true".to_string(),
            expected_message: "unknown operator: -BOOLEAN".to_string(),
        },
        Test {
            input: "true + false;".to_string(),
            expected_message: "unknown operator: BOOLEAN + BOOLEAN".to_string(),
        },
        Test {
            input: "5; true + false; 5".to_string(),
            expected_message: "unknown operator: BOOLEAN + BOOLEAN".to_string(),
        },
        Test {
            input: "if (10 > 1) { true + false; }".to_string(),
            expected_message: "unknown operator: BOOLEAN + BOOLEAN".to_string(),
        },
        Test {
            input: r#"
if (10 > 1) {
    if (10 > 1) {
        return true + false;
    }

    return 1;
}
"#
            .to_string(),
            expected_message: "unknown operator: BOOLEAN + BOOLEAN".to_string(),
        },
        Test {
            input: "foobar".to_string(),
            expected_message: "identifier not found: foobar".to_string(),
        },
        Test {
            input: r#""Hello" - "World""#.to_string(),
            expected_message: "unknown operator: STRING - STRING".to_string(),
        },
    ];

    for tt in tests {
        let evaluated = test_eval(tt.input);

        match evaluated {
            Ok(value) => {
                eprintln!("no error object returned. got = {:?}", value);
                continue;
            }
            Err(err) => {
                if format!("{}", err) != tt.expected_message {
                    eprintln!(
                        "wrong error message. expected = {}, got = {}",
                        tt.expected_message,
                        format!("{}", err)
                    )
                }
            }
        }
    }
    Ok(())
}

fn test_let_statements() -> anyhow::Result<()> {
    struct Test {
        input: String,
        expected: i64,
    }

    let tests = vec![
        Test {
            input: "let a = 5; a;".to_string(),
            expected: 5,
        },
        Test {
            input: "let a = 5 * 5; a;".to_string(),
            expected: 25,
        },
        Test {
            input: "let a = 5; let b = a; b;".to_string(),
            expected: 5,
        },
        Test {
            input: "let a = 5; let b = a; let c = a + b + 5; c;".to_string(),
            expected: 15,
        },
    ];

    for tt in tests {
        let ret = test_integer_object(test_eval(tt.input)?, tt.expected)?;
        if !ret {
            eprintln!("test integer object error");
        }
    }

    Ok(())
}

fn test_function_object() -> anyhow::Result<()> {
    let input = "fn(x) { x + 2; };";

    let evaluated = test_eval(input.to_string())?;

    let value = match evaluated {
        Object::Function(fn_value) => fn_value,
        _ => {
            panic!("object is no function. got = {}", evaluated);
        }
    };

    if value.parameters.len() != 1 {
        eprintln!(
            "function has wrong parameters. parameters = {:?}",
            value.parameters
        );
    }

    if format!("{}", value.parameters[0]) != "x" {
        eprintln!("parameter is no 'x'. got = {:?}", value.parameters[0]);
    }

    let expected_body = "(x + 2);";

    if format!("{}", value.body) != expected_body {
        eprintln!("body is not {}. got = {}", expected_body, value.body);
    }

    Ok(())
}

fn test_function_application() -> anyhow::Result<()> {
    struct Test {
        input: String,
        expected: i64,
    }

    let tests = vec![
        Test {
            input: "let identity = fn(x) { x; }; identity(5);".to_string(),
            expected: 5,
        },
        Test {
            input: "let identity = fn(x) { return x; }; identity(5);".to_string(),
            expected: 5,
        },
        Test {
            input: "let double = fn(x) { return x * 2; }; double(5);".to_string(),
            expected: 10,
        },
        Test {
            input: "let add = fn(x, y) { return x + y; }; add(5, 5);".to_string(),
            expected: 10,
        },
        Test {
            input: "let add = fn(x, y) { x + y; }; add(5 + 5, add(5, 5));".to_string(),
            expected: 20,
        },
        Test {
            input: "fn(x) { x; }(5)".to_string(),
            expected: 5,
        },
    ];

    for tt in tests {
        let ret = test_integer_object(test_eval(tt.input)?, tt.expected)?;
        if !ret {
            eprintln!("test integer object failed");
        }
    }

    Ok(())
}

fn test_closures() -> anyhow::Result<()> {
    let input = r#"
let newAddr = fn(x) {
    fn(y) { x + y };
};
let addTwo = newAddr(2);
addTwo(2);"#
        .to_string();

    let ret = test_integer_object(test_eval(input)?, 4)?;
    if !ret {
        eprintln!("test integer object failed");
    }

    Ok(())
}


fn test_string_literal() -> anyhow::Result<()>{
    let input = r#""Hello World!""#;
    let evaluated = test_eval(input.to_string())?;

    let str_lit = match evaluated {
        Object::String(string_lit) => string_lit,
        _ => {
            panic!("object is not String. got = {}", evaluated);
        }
    };

    println!("test string literal = {:?}", str_lit);

    if str_lit.value != "Hello World!" {
        eprintln!("String has wrong value. got = {}", str_lit.value);
    }
    Ok(())
}

fn test_string_concatenation() -> anyhow::Result<()>{
    let input = r#""Hello" + " " + "World!""#;

    let evaluated = test_eval(input.to_string())?;
    let str_lit = match evaluated {
        Object::String(string_lit) => string_lit,
        _ => {
            return Err(anyhow::anyhow!(format!("object is not String. got = {}", evaluated)));
        }
    };

    if str_lit.value != "Hello World!" {
        return Err(anyhow::anyhow!(format!("String has wrong value. got = {}", str_lit.value)));
    }

    Ok(())
}

fn test_string_not_equal() -> anyhow::Result<()>{
    let input = r#""Hello" != "World!""#;

    let evaluated = test_eval(input.to_string())?;
    let bool_str = match evaluated {
        Object::Boolean(value) => value,
        _ => {
            return Err(anyhow::anyhow!(format!("object is not Boolean. got = {}", evaluated)));
        }
    };

    if bool_str.value != true {
        return Err(anyhow::anyhow!(format!("Boolean has wrong value. got = {}", bool_str.value)));
    }

    Ok(())
}

fn test_string_equal() -> anyhow::Result<()>{
    let input = r#""Hello" == "Hello""#;

    let evaluated = test_eval(input.to_string())?;
    let bool_str = match evaluated {
        Object::Boolean(value) => value,
        _ => {
            return Err(anyhow::anyhow!(format!("object is not Boolean. got = {}", evaluated)));
        }
    };

    if bool_str.value != true {
        return Err(anyhow::anyhow!(format!("Boolean has wrong value. got = {}", bool_str.value)));
    }

    Ok(())
}


fn test_builtin_functions() -> anyhow::Result<()> {
    struct  Test {
        input: String,
        expected: Box<dyn Interface>,
    }

    let tests = vec! [
        Test {
            input: r#"len("")"#.to_string(),
            expected: Box::new(0),
        },
        Test {
            input: r#"len("four")"#.to_string(),
            expected: Box::new(4),
        },
        Test {
            input: r#"len("hello world")"#.to_string(),
            expected: Box::new(11),
        },
        Test {
            input: r#"len(1)"#.to_string(),
            expected: Box::new( "argument to `len` not supported, got INTEGER"),
        },
        Test {
            input: r#"len("one", "two")"#.to_string(),
            expected: Box::new( "wrong number of arguments. got=2, want=1".to_string()),
        },
    ];


    for tt in tests {
        let evaluated = test_eval(tt.input);
        println!("[test_builtin_functions] evaluated = {:?}", evaluated);
        let t = tt.expected.as_any().type_id();
        if TypeId::of::<i64>() == t {
            let value = tt.expected
                .as_any()
                .downcast_ref::<i64>()
                .expect("downcast_ref error");
            test_integer_object(evaluated?, value.clone())?;
        } else if TypeId::of::<String>() == t {
            let value = tt.expected
                .as_any()
                .downcast_ref::<String>()
                .expect("downcast_ref error");
            if let Err(error) = evaluated {
                let error_obj_message = format!("{}", error);
                if error_obj_message.as_str() != value.as_str() {
                    eprintln!("wrong error message. expected: {}, got = {}", value, error_obj_message);
                }
            } else {
                eprintln!("object is not Error. got = {}", evaluated?);
            }
        }
        else if TypeId::of::<&str>() == t {
            let value = tt.expected
                .as_any()
                .downcast_ref::<&str>()
                .expect("downcast_ref error");

            if let Err(error) = evaluated {
                let error_obj_message = format!("{}", error);
                if &error_obj_message != value {
                    eprintln!("wrong error message. expected: {}, got = {}", value, error_obj_message);
                }
            } else {
                eprintln!("object is not Error. got = {}", evaluated?);
            }

        }
        // else if TypeId::of::<bool>() == t {
        //     let value = tt.expected
        //         .as_any()
        //         .downcast_ref::<bool>()
        //         .expect("downcast_ref error");
        //
        // }
        else {
            eprintln!("type of exp not handle.");
        }
    }

    Ok(())
}

trait Interface {
    fn as_any(&self) -> &dyn Any;
}

impl Interface for i64 {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl From<i64> for Box<dyn Interface> {
    fn from(value: i64) -> Self {
        Box::new(value)
    }
}

impl Interface for bool {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl From<bool> for Box<dyn Interface> {
    fn from(value: bool) -> Self {
        Box::new(value)
    }
}

impl Interface for () {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl From<()> for Box<dyn Interface> {
    fn from(val: ()) -> Self {
        Box::new(val)
    }
}


impl Interface for String {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl From<String>  for Box<dyn Interface> {
    fn from(val: String) -> Self {
        Box::new(val)
    }
}

impl Interface for &'static str {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl From<&'static str>  for Box<dyn Interface> {
    fn from(val: &'static str) -> Self {
        Box::new(val)
    }
}


#[test]
fn test_test_eval_integer_expression() {
    let ret = test_eval_integer_expression();
    println!("test_eval_integer_expression : ret = {:?}", ret);
}

#[test]
fn test_test_eval_boolean_expression() {
    let ret = test_eval_boolean_expression();
    println!("test_eval_boolean_expression : ret = {:?}", ret);
}

#[test]
fn test_test_bang_operator() {
    let ret = test_bang_operator();
    println!("test_bang_operator : ret = {:?}", ret);
}

#[test]
fn test_test_if_else_expressions() {
    let ret = test_if_else_expressions();
    println!("test_if_else_expressions : ret = {:?}", ret);
}

#[test]
fn test_test_return_statements() {
    let ret = test_return_statements();
    println!("test_test_return_statements: ret = {:?}", ret);
}

#[test]
fn test_test_error_handling() {
    let ret = test_error_handling();
    println!("test_error_handling: ret = {:?}", ret);
}

#[test]
fn test_test_let_statements() {
    let ret = test_let_statements();
    println!("test_let_statements: ret = {:?}", ret);
}

#[test]
fn test_test_function_object() {
    let ret = test_function_object();
    println!("test_function_object: ret = {:?}", ret);
}

#[test]
fn test_test_function_application() {
    let ret = test_function_application();
    println!("test_function_application: ret = {:?}", ret);
}

#[test]
fn test_test_closures() {
    let ret = test_closures();
    println!("test_closures : ret = {:?}", ret);
}


#[test]
fn test_test_string_literal() {
    let ret = test_string_literal();
    println!("test_string_literal: ret = {:?}", ret);
}

#[test]
fn test_test_string_concatenation() {
    let ret = test_string_concatenation();
    println!("test_string_concatenation: ret = {:?}", ret);
}

#[test]
fn test_test_string_not_equal() {
    let ret = test_string_not_equal();
    println!("test_string_not_equal: ret = {:?}", ret);
}

#[test]
fn test_test_string_equal() {
    let ret = test_string_equal();
    println!("test_string_equal: ret = {:?}", ret);
}

#[test]
fn test_test_builtin_functions() {
    let ret = test_builtin_functions();
    println!("test_builtin_functions: ret = {:?}", ret);
}