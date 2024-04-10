/*
 * @Author: elijte 2681994981@qq.com
 * @Date: 2024-04-08 19:41:58
 * @LastEditors: elijte 2681994981@qq.com
 * @LastEditTime: 2024-04-10 10:57:36
 * @FilePath: \rust-rustlings-2024-spring-eljrte\exercises\iterators\iterators1.rs
 * @Description: 这是默认设置,请设置`customMade`, 打开koroFileHeader查看配置 进行设置: https://github.com/OBKoro1/koro1FileHeader/wiki/%E9%85%8D%E7%BD%AE
 */
// iterators1.rs
//
// When performing operations on elements within a collection, iterators are
// essential. This module helps you get familiar with the structure of using an
// iterator and how to go through elements within an iterable collection.
//
// Make me compile by filling in the `???`s
//
// Execute `rustlings hint iterators1` or use the `hint` watch subcommand for a
// hint.


fn main() {
    let my_fav_fruits = vec!["banana", "custard apple", "avocado", "peach", "raspberry"];

    let mut my_iterable_fav_fruits = my_fav_fruits.into_iter();   // TODO: Step 1

    assert_eq!(my_iterable_fav_fruits.next(), Some("banana"));
    assert_eq!(my_iterable_fav_fruits.next(), Some("custard apple"));     // TODO: Step 2
    assert_eq!(my_iterable_fav_fruits.next(), Some("avocado"));
    assert_eq!(my_iterable_fav_fruits.next(), Some("peach"));     // TODO: Step 3
    assert_eq!(my_iterable_fav_fruits.next(), Some("raspberry"));
    assert_eq!(my_iterable_fav_fruits.next(), None);     // TODO: Step 4
}
