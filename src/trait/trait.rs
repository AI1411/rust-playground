trait TreasureBox {
    fn open(&self, key_no: i32) -> bool;
    fn check(&self);
}

struct JewelryBox {
    price: i32,
    key_no: i32,
}

impl TreasureBox for JewelryBox {
    fn open(&self, key_no: i32) -> bool {
        self.key_no == key_no
    }
    fn check(&self) {
        println!("宝石箱だった！金貨{}枚入手", self.price);
    }
}

struct TrapBox {
    damage: i32,
}

impl TreasureBox for TrapBox {
    fn open(&self, _key: i32) -> bool {
        return true;
    }
    fn check(&self) {
        println!("罠だった。{}のダメージ", self.damage);
    }
}

fn open_box(t_box: &impl TreasureBox, key_no: i32) {
    if !t_box.open(key_no) {
        println!("鍵が合わず宝箱が開きません");
        return;
    }
    t_box.check();
}

fn main() {
    let box1 = JewelryBox { price: 30, key_no: 1 };
    let box2 = TrapBox { damage: 3 };
    let box3 = JewelryBox { price: 20, key_no: 2 };
    let my_key = 2;

    open_box(&box1, my_key);
    open_box(&box2, my_key);
    open_box(&box3, my_key);
}
