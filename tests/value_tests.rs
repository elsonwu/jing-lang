use jing::*;

#[test]
fn test_value_display() {
    assert_eq!(format!("{}", Value::Nil), "nil");
    assert_eq!(format!("{}", Value::Bool(true)), "true");
    assert_eq!(format!("{}", Value::Bool(false)), "false");
    assert_eq!(format!("{}", Value::Number(42.0)), "42");
    assert_eq!(format!("{}", Value::Number(3.14)), "3.14");
    assert_eq!(format!("{}", Value::String("hello".to_string())), "hello");

    let func = Value::Function {
        name: "test".to_string(),
        arity: 2,
        chunk_start: 0,
    };
    assert_eq!(format!("{}", func), "<fn test(2 args)>");
}

#[test]
fn test_value_truthiness() {
    assert!(!Value::Nil.is_truthy());
    assert!(Value::Nil.is_falsy());

    assert!(Value::Bool(true).is_truthy());
    assert!(!Value::Bool(true).is_falsy());

    assert!(!Value::Bool(false).is_truthy());
    assert!(Value::Bool(false).is_falsy());

    assert!(Value::Number(42.0).is_truthy());
    assert!(Value::Number(0.0).is_truthy()); // 0 is truthy in Jing
    assert!(Value::Number(-1.0).is_truthy());

    assert!(Value::String("hello".to_string()).is_truthy());
    assert!(Value::String("".to_string()).is_truthy()); // Empty string is truthy

    let func = Value::Function {
        name: "test".to_string(),
        arity: 0,
        chunk_start: 0,
    };
    assert!(func.is_truthy());
}

#[test]
fn test_value_type_names() {
    assert_eq!(Value::Nil.type_name(), "nil");
    assert_eq!(Value::Bool(true).type_name(), "bool");
    assert_eq!(Value::Number(42.0).type_name(), "number");
    assert_eq!(Value::String("hello".to_string()).type_name(), "string");

    let func = Value::Function {
        name: "test".to_string(),
        arity: 0,
        chunk_start: 0,
    };
    assert_eq!(func.type_name(), "function");
}

#[test]
fn test_value_to_string() {
    assert_eq!(Value::String("hello".to_string()).to_string(), "hello");
    assert_eq!(Value::Number(42.0).to_string(), "42");
    assert_eq!(Value::Bool(true).to_string(), "true");
    assert_eq!(Value::Nil.to_string(), "nil");
}

#[test]
fn test_value_to_number() {
    assert_eq!(Value::Number(42.0).to_number().unwrap(), 42.0);
    assert_eq!(Value::String("3.14".to_string()).to_number().unwrap(), 3.14);
    assert_eq!(Value::String("42".to_string()).to_number().unwrap(), 42.0);

    // Error cases
    assert!(Value::String("invalid".to_string()).to_number().is_err());
    assert!(Value::Bool(true).to_number().is_err());
    assert!(Value::Nil.to_number().is_err());
}

#[test]
fn test_value_addition() {
    // Number addition
    let result = Value::Number(5.0).add(&Value::Number(3.0)).unwrap();
    assert_eq!(result, Value::Number(8.0));

    // String concatenation
    let result = Value::String("Hello, ".to_string())
        .add(&Value::String("World!".to_string()))
        .unwrap();
    assert_eq!(result, Value::String("Hello, World!".to_string()));

    // String + other types
    let result = Value::String("Number: ".to_string())
        .add(&Value::Number(42.0))
        .unwrap();
    assert_eq!(result, Value::String("Number: 42".to_string()));

    let result = Value::Number(42.0)
        .add(&Value::String(" is the answer".to_string()))
        .unwrap();
    assert_eq!(result, Value::String("42 is the answer".to_string()));

    // Error cases
    assert!(Value::Bool(true).add(&Value::Bool(false)).is_err());
    assert!(Value::Nil.add(&Value::Number(42.0)).is_err());
}

#[test]
fn test_value_subtraction() {
    let result = Value::Number(10.0).subtract(&Value::Number(3.0)).unwrap();
    assert_eq!(result, Value::Number(7.0));

    // Error cases
    assert!(Value::String("hello".to_string())
        .subtract(&Value::Number(1.0))
        .is_err());
    assert!(Value::Bool(true).subtract(&Value::Bool(false)).is_err());
}

#[test]
fn test_value_multiplication() {
    let result = Value::Number(6.0).multiply(&Value::Number(7.0)).unwrap();
    assert_eq!(result, Value::Number(42.0));

    // Error cases
    assert!(Value::String("hello".to_string())
        .multiply(&Value::Number(2.0))
        .is_err());
    assert!(Value::Bool(true).multiply(&Value::Bool(false)).is_err());
}

#[test]
fn test_value_division() {
    let result = Value::Number(15.0).divide(&Value::Number(3.0)).unwrap();
    assert_eq!(result, Value::Number(5.0));

    // Division by zero
    assert!(Value::Number(10.0).divide(&Value::Number(0.0)).is_err());

    // Error cases
    assert!(Value::String("hello".to_string())
        .divide(&Value::Number(2.0))
        .is_err());
    assert!(Value::Bool(true).divide(&Value::Bool(false)).is_err());
}

#[test]
fn test_value_modulo() {
    let result = Value::Number(17.0).modulo(&Value::Number(5.0)).unwrap();
    assert_eq!(result, Value::Number(2.0));

    // Modulo by zero
    assert!(Value::Number(10.0).modulo(&Value::Number(0.0)).is_err());

    // Error cases
    assert!(Value::String("hello".to_string())
        .modulo(&Value::Number(2.0))
        .is_err());
    assert!(Value::Bool(true).modulo(&Value::Bool(false)).is_err());
}

#[test]
fn test_value_negation() {
    let result = Value::Number(42.0).negate().unwrap();
    assert_eq!(result, Value::Number(-42.0));

    let result = Value::Number(-3.14).negate().unwrap();
    assert_eq!(result, Value::Number(3.14));

    // Error cases
    assert!(Value::String("hello".to_string()).negate().is_err());
    assert!(Value::Bool(true).negate().is_err());
    assert!(Value::Nil.negate().is_err());
}

#[test]
fn test_value_not() {
    assert_eq!(Value::Bool(true).not(), Value::Bool(false));
    assert_eq!(Value::Bool(false).not(), Value::Bool(true));
    assert_eq!(Value::Nil.not(), Value::Bool(true));
    assert_eq!(Value::Number(42.0).not(), Value::Bool(false));
    assert_eq!(Value::String("hello".to_string()).not(), Value::Bool(false));
}

#[test]
fn test_value_equality() {
    assert!(Value::Nil.equals(&Value::Nil));
    assert!(Value::Bool(true).equals(&Value::Bool(true)));
    assert!(Value::Bool(false).equals(&Value::Bool(false)));
    assert!(!Value::Bool(true).equals(&Value::Bool(false)));

    assert!(Value::Number(42.0).equals(&Value::Number(42.0)));
    assert!(!Value::Number(42.0).equals(&Value::Number(43.0)));

    assert!(Value::String("hello".to_string()).equals(&Value::String("hello".to_string())));
    assert!(!Value::String("hello".to_string()).equals(&Value::String("world".to_string())));

    // Different types should not be equal
    assert!(!Value::Number(42.0).equals(&Value::String("42".to_string())));
    assert!(!Value::Bool(true).equals(&Value::Number(1.0)));
    assert!(!Value::Nil.equals(&Value::Bool(false)));
}

#[test]
fn test_value_comparisons() {
    // Number comparisons
    assert!(Value::Number(5.0).less_than(&Value::Number(10.0)).unwrap());
    assert!(!Value::Number(10.0).less_than(&Value::Number(5.0)).unwrap());
    assert!(!Value::Number(5.0).less_than(&Value::Number(5.0)).unwrap());

    assert!(Value::Number(10.0)
        .greater_than(&Value::Number(5.0))
        .unwrap());
    assert!(!Value::Number(5.0)
        .greater_than(&Value::Number(10.0))
        .unwrap());
    assert!(!Value::Number(5.0)
        .greater_than(&Value::Number(5.0))
        .unwrap());

    // String comparisons
    assert!(Value::String("apple".to_string())
        .less_than(&Value::String("banana".to_string()))
        .unwrap());
    assert!(!Value::String("banana".to_string())
        .less_than(&Value::String("apple".to_string()))
        .unwrap());

    assert!(Value::String("banana".to_string())
        .greater_than(&Value::String("apple".to_string()))
        .unwrap());
    assert!(!Value::String("apple".to_string())
        .greater_than(&Value::String("banana".to_string()))
        .unwrap());

    // Error cases - cannot compare different types
    assert!(Value::Number(5.0)
        .less_than(&Value::String("10".to_string()))
        .is_err());
    assert!(Value::Bool(true).greater_than(&Value::Number(1.0)).is_err());
}

#[test]
fn test_environment() {
    let mut env = Environment::new();

    // Test define and get
    env.define("x".to_string(), Value::Number(42.0));
    assert_eq!(env.get("x").unwrap(), Value::Number(42.0));

    // Test undefined variable
    assert!(env.get("undefined").is_err());

    // Test set existing variable
    env.set("x", Value::Number(100.0)).unwrap();
    assert_eq!(env.get("x").unwrap(), Value::Number(100.0));

    // Test set undefined variable
    assert!(env.set("undefined", Value::Number(1.0)).is_err());

    // Test scoping
    env.push_scope();
    env.define("y".to_string(), Value::String("inner".to_string()));
    assert_eq!(env.get("y").unwrap(), Value::String("inner".to_string()));
    assert_eq!(env.get("x").unwrap(), Value::Number(100.0)); // Can still access outer scope

    env.pop_scope();
    assert!(env.get("y").is_err()); // Variable from inner scope is gone
    assert_eq!(env.get("x").unwrap(), Value::Number(100.0)); // Outer scope still accessible

    // Test shadowing
    env.push_scope();
    env.define("x".to_string(), Value::String("shadowed".to_string()));
    assert_eq!(env.get("x").unwrap(), Value::String("shadowed".to_string()));

    env.pop_scope();
    assert_eq!(env.get("x").unwrap(), Value::Number(100.0)); // Original value restored
}

#[test]
fn test_environment_edge_cases() {
    let mut env = Environment::new();

    // Cannot pop the global scope
    env.pop_scope(); // Should not panic or error, just not do anything

    // Can still use the environment after trying to pop global scope
    env.define("test".to_string(), Value::Bool(true));
    assert_eq!(env.get("test").unwrap(), Value::Bool(true));
}
