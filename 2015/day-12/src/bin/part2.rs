use serde_json::Value;

fn main() {
    let input: Value = serde_json::from_str(include_str!("../../input.json")).unwrap();
    println!("{}", solve(&input));
}

fn solve(input: &Value) -> i64 {
    match input {
        Value::Array(x) => x.iter().map(|val| solve(val)).sum::<i64>(),
        Value::String(_) | Value::Null | Value::Bool(_) => 0,
        Value::Object(m) => {
            for (_k,v) in m {
              if let Value::String(x) = v {
                if x == "red" {
                  return 0;
                }
              }
            }
            m.iter().map(|(_k, v)| solve(v)).sum::<i64>()
        }
        Value::Number(x) => x.as_i64().unwrap(),
    }
}
