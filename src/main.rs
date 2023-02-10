fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    #[test]
    fn dummy() {
        assert_eq!(2 + 2, 4);
    }
}
