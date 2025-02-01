pub mod structs {
    use serde::{Deserialize, Serialize};

    #[derive(Clone)]
    pub struct User{
        pub uid: String,
        pub email: String,
        pub password: String,
        pub role: String    
    }
    
    
    #[derive(Deserialize)]
    pub struct LoginRequest {
        pub email: String,
        pub password: String,
    }
    
    #[derive(Serialize)]
    pub struct LoginResponse{
        pub token: String,
    }
}