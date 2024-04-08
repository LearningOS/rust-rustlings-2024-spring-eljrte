/*
 * @Author: elijte 2681994981@qq.com
 * @Date: 2024-04-08 19:41:58
 * @LastEditors: elijte 2681994981@qq.com
 * @LastEditTime: 2024-04-08 20:43:49
 * @FilePath: \rust-rustlings-2024-spring-eljrte\exercises\primitive_types\primitive_types5.rs
 * @Description: 这是默认设置,请设置`customMade`, 打开koroFileHeader查看配置 进行设置: https://github.com/OBKoro1/koro1FileHeader/wiki/%E9%85%8D%E7%BD%
 */
// primitive_types5.rs
//
// Destructure the `cat` tuple so that the println will work.
//
// Execute `rustlings hint primitive_types5` or use the `hint` watch subcommand
// for a hint.


fn main() {
    let cat = ("Furry McFurson", 3.5);
    let (name,age)/* your pattern here */ = cat;

    println!("{} is {} years old.", name, age);
}
