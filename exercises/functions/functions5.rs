/*
 * @Author: elijte 2681994981@qq.com
 * @Date: 2024-04-08 19:41:58
 * @LastEditors: elijte 2681994981@qq.com
 * @LastEditTime: 2024-04-08 20:46:12
 * @FilePath: \rust-rustlings-2024-spring-eljrte\exercises\functions\functions5.rs
 * @Description: 这是默认设置,请设置`customMade`, 打开koroFileHeader查看配置 进行设置: https://github.com/OBKoro1/koro1FileHeader/wiki/%E9%85%8D%E7%BD%AE
 */
// functions5.rs
//
// Execute `rustlings hint functions5` or use the `hint` watch subcommand for a
// hint.


fn main() {
    let answer = square(3);
    println!("The square of 3 is {}", answer);
}

fn square(num: i32) -> i32 {
    num * num
}
