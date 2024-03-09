#[test]
fn test_start() {
    let args = vec!["runtime", "-u".to_string(), "https://www.youtube.com/watch?v=6g4dkBF5anU".to_string()];
    let result = start(&args);
    assert_eq!(result, Ok(()));
}

#[test]
fn test_param() {
    let args = vec!["runtime".to_string(), "-u".to_string(), "https://www.youtube.com/watch?v=6g4dkBF5anU".to_string()];
    let args2 = vec!["runtime".to_string(), "-s".to_string(), "shirinko".to_string()];
    let result = create_parameter_list(args);
    let result2 = create_parameter_list(args2);

    // Should passed
    assert_eq!(result.0.len(), 1);
    assert_eq!(result.0[0].parameter_type, ParameterType::URL);
    assert_eq!(result.0[0].value, "https://www.youtube.com/watch?v=6g4dkBF5anU");

    assert_eq!(result2.0.len(), 1);
    assert_eq!(result2.0[0].parameter_type, ParameterType::Search);
    assert_eq!(result2.0[0].value, "shirinko");

    // Should failed
    assert_panic!(create_parameter_list(vec!["runtime".to_string(), "-u".to_string(),
        "https://www.youtube.com/watch?v=6g4dkBF5anU".to_string(), "-f".to_string(),
         "nai".to_string()]));
    assert_panic!(create_parameter_list(vec!["runtime".to_string(), "-u".to_string(),
        "https://www.youtube.com/watch?v=6g4dkBF5anU".to_string(), "-f".to_string(),
         "mp4".to_string(), "--invalid-args".to_string()]));
    assert_panic!(create_parameter_list(vec!["runtime".to_string(), "-u".to_string()]));
    assert_panic!(create_parameter_list(vec!["runtime".to_string(), "-s".to_string(), "https://www.youtube.com/watch?v=6g4dkBF5anU".to_string()]));
}