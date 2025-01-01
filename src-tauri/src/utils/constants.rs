use lazy_static::lazy_static;

lazy_static! {
    pub static ref DB_URL: String = String::from("sqlite://app.db");
}
//https://msoln-server.onrender.com/

lazy_static! {
    pub static ref API: String = String::from("http://localhost:8000/");
}

//localhost:
