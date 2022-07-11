// #[derive(Debug)]
// pub enum Method {
//     Get,
//     Post,
//     Uninitialized
// }

// impl From<&str> for Method {
//     fn from(s: &str) -> Method {
//         match s {
//             "GET" => Method::Get,
//             "POST" => Method::Post,
//             _ => Method::Uninitialized,
//         }
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_method_info() {
//         let m: Method = "GET".into();
//         assert_eq!(m, Method::Get);
//     }
// }