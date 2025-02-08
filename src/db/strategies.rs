#[derive(Debug, Serialize, Deserialize)]
pub struct Strategy {
    pub id: i64,
    pub user_id: i64,
    pub strategy_id: i64,
    pub expiration: Option<String>,  // DATE como String
    pub amount: Option<f64>,
    pub start_date: Option<String>,  // DATE como String
    pub enabled: i64,  // Cambiado de bool a i64
} 