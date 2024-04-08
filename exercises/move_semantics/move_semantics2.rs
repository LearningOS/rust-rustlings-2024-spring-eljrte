/*
 * @Author: elijte 2681994981@qq.com
 * @Date: 2024-04-08 19:41:58
 * @LastEditors: elijte 2681994981@qq.com
 * @LastEditTime: 2024-04-08 21:35:01
 * @FilePath: \rust-rustlings-2024-spring-eljrte\exercises\move_semantics\move_semantics2.rs
 * @Description: 这是默认设置,请设置`customMade`, 打开koroFileHeader查看配置 进行设置: https://github.com/OBKoro1/koro1FileHeader/wiki/%E9%85%8D%E7%BD%AE
 */
// move_semantics2.rs
//
// Expected output:
// vec0 has length 3, with contents `[22, 44, 66]`
// vec1 has length 4, with contents `[22, 44, 66, 88]`
//
// Execute `rustlings hint move_semantics2` or use the `hint` watch subcommand
// for a hint.


fn main() {
    let mut vec0 = Vec::new();
    
    //法一 多整几个
    // let mut vec0 = fill_vec(vec0);
    // println!("{} has length {}, with contents: `{:?}`", "vec0", vec0.len(), vec0);
    // let mut vec1 = vec0;
    // vec1.push(88);

    let mut vec1 = fill_vec(&mut vec0);
    println!("{} has length {}, with contents: `{:?}`", "vec0", vec0.len(), vec0);

    vec1.push(88);

    println!("{} has length {}, with contents `{:?}`", "vec1", vec1.len(), vec1);
}

fn fill_vec(vec: &mut Vec<i32>) -> Vec<i32> {
    let mut vec1=Vec::new();

    vec1.push(22);
    vec1.push(44);
    vec1.push(66);

    *vec=vec1.clone();
    vec1
}
