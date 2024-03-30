fn main() -> Result<(), chinese_id::ValidationError> {
    let id = std::env::args().nth(1).expect("missing id");
    let result = chinese_id::validate(&id)?;
    println!("{:?}", result);
    Ok(())
}
