/*
 * @Author: elijte 2681994981@qq.com
 * @Date: 2024-04-08 19:41:58
 * @LastEditors: elijte 2681994981@qq.com
 * @LastEditTime: 2024-04-08 20:16:23
 * @FilePath: \rust-rustlings-2024-spring-eljrte\exercises\if\if1.rs
 * @Description: 这是默认设置,请设置`customMade`, 打开koroFileHeader查看配置 进行设置: https://github.com/OBKoro1/koro1FileHeader/wiki/%E9%85%8D%E7%BD%AE
 */
// if1.rs
//
// Execute `rustlings hint if1` or use the `hint` watch subcommand for a hint.


pub fn bigger(a: i32, b: i32) -> i32 {
    if a>b {
        a
    }
    else{
        b
    }
}

// Don't mind this for now :)
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ten_is_bigger_than_eight() {
        assert_eq!(10, bigger(10, 8));
    }

    #[test]
    fn fortytwo_is_bigger_than_thirtytwo() {
        assert_eq!(42, bigger(32, 42));
    }
}
