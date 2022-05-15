struct Item(String, i64);

fn main() {
    let banana = Item("バナナ".to_string(), 300);
    let apple = Item("りんご".to_string(), 200);
    let orange = Item("オレンジ".to_string(), 400);

    let items = vec![banana, apple, orange];
    let total = print_and_sum_items(&items);
    println!("total = {}", total);
}

fn print_tuple(item: &Item) {
    println!("{}を{}円で購入", item.0, item.1);
}

fn print_and_sum_items(items: &Vec<Item>) -> i64 {
    let mut total = 0;
    for item in items {
        print_tuple(&item);
        total += item.1;
    }
    total
}
