#[cfg(test)]
mod tests {
    #[test]
    pub fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

pub mod hello {
    pub fn hello_world() {
        println!("hello world!");
    }
}
