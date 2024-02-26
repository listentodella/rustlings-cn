// errors4.rs
// 执行 `rustlings hint errors4` 或在观察模式下使用 `hint` 子命令来获取提示。


#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
        if value > 0 {
            Ok(PositiveNonzeroInteger(value as u64))
        } else if value < 0 {
            Err(CreationError::Negative)
        } else {
            Err(CreationError::Zero)
        }
    }
}

#[test]
fn test_creation() {
    assert!(PositiveNonzeroInteger::new(10).is_ok());
    assert_eq!(
        Err(CreationError::Negative),
        PositiveNonzeroInteger::new(-10)
    );
    assert_eq!(Err(CreationError::Zero), PositiveNonzeroInteger::new(0));
}
