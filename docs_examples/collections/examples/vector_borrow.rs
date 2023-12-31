// ❌
fn main() {
    let mut v = vec![1, 2, 3, 4, 5];

    let first = &v[0]; // 不變參考

    v.push(6); // 可變參考

    println!("第一個元素是：{first}");
}
// ⭐️在向量後方新增元素時，如果當前向量的空間不夠再塞入另一個值的話，可能會需要配置新的記憶體並複製舊的元素到新的空間中。
// ⭐️這樣一來，第一個元素的索引可能就會指向已經被釋放的記憶體，借用規則會防止程式遇到這樣的情形。

// ✅
fn main() {
    let mut v = vec![1, 2, 3, 4, 5];

    {
        let first = &v[0]; // 不變參考
        println!("第一個元素是：{first}");
    }

    v.push(6); // 可變參考

    println!("向量 v: {:?}", v);
}
