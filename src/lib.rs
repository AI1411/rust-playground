pub mod sorting;
pub mod searching;

#[cfg(test)]
mod tests {
    #[test]
   fn array_test() {
        let a1 = [100, 200, 300];
        let a2 = [100, 200, 300];

        assert_eq!(a1, a2);

        let a3 = [
            "apple".to_string(), "banana".to_string()
        ];
        let a4 = [
            String::from("apple"),
            String::from("banana")
        ];

        assert_eq!(a3, a4);
    }

    #[test]
    fn vec_test() {
        let v1 = vec!["apple", "banana", "orange"];
        let mut v2: Vec<&str> = Vec::new();
        v2.push("apple");
        v2.push("banana");
        v2.push("orange");

        assert_eq!(v1, v2);
    }

    #[derive(Debug,PartialEq)]
    struct GItem {
        name: String,
        price: i64,
    }

    #[test]
    fn item_test() {
        let apple1 = GItem {
            name: String::from("apple").to_string(),
            price: 2400,
        };
        let mut apple2 = GItem {
            name: "apple".to_string(),
            price: 0,
        };
        apple2.price = 2400;

        assert_eq!(apple1.name, apple2.name);
        assert_eq!(apple2.price, apple2.price);

        assert_eq!(apple1, apple2);
    }
}
