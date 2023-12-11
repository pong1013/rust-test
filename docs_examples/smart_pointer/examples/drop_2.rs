struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("釋放 CustomSmartPointer 的資料 `{}`！", self.data);
    }
}

fn main() {
    let c = CustomSmartPointer {
        data: String::from("some data"),
    };
    println!("MySmartPointer created.");

    // ❌ COMPLIE ERROR: explicit use of destructor method
    // c.drop();

    // ✅ Automatically imported from `std::mem::drop`
    drop(c);

    println!("MySmartPointer dropped before the end of main.");
}
