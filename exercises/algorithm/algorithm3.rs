/*
 * @Author: elijte 2681994981@qq.com
 * @Date: 2024-04-08 19:41:58
 * @LastEditors: elijte 2681994981@qq.com
 * @LastEditTime: 2024-04-13 10:38:27
 * @FilePath: \rust-rustlings-2024-spring-eljrte\exercises\algorithm\algorithm3.rs
 * @Description: 这是默认设置,请设置`customMade`, 打开koroFileHeader查看配置 进行设置: https://github.com/OBKoro1/koro1FileHeader/wiki/%E9%85%8D%E7%BD%AE
 */

// I AM NOT DONE
/*
	sort
	This problem requires you to implement a sorting algorithm
	you can use bubble sorting, insertion sorting, heap sorting, etc.
*/

fn sift_down<T: PartialOrd>(array: &mut [T], mut root: usize, end: usize) {
    let mut child;
    while 2 * root + 1 <= end { // 当前节点有左子节点
        child = 2 * root + 1; // 左子节点的位置
        if child + 1 <= end && array[child] < array[child + 1] {
            child += 1; // 右子节点存在且值大于左子节点，则定位到右子节点
        }
        if array[root] < array[child] {
            array.swap(root, child); // 如果当前节点小于子节点的值，则交换它们
            root = child; // 继续下沉调整
        } else {
            break; // 如果当前节点大于等于子节点，无需继续调整
        }
    }
}
fn sort<T:PartialOrd>(array: &mut [T]){
	//TODO
    
    //bubble sorting
    // for i in 0..array.len(){
    //     for j in i..array.len(){
    //         if array[i] > array[j]{
    //             array.swap(i,j);
    //         }
    //     }
    // }

    //heap sorting
    let len = array.len();
    if len < 2{
        return ;
    }

    for start in (0..len/2).rev(){
        sift_down(array,start,len-1);
    }

    for end in(1..len).rev(){
        array.swap(0,end);
        sift_down(array,0,end-1);
    }

}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_1() {
        let mut vec = vec![37, 73, 57, 75, 91, 19, 46, 64];
        sort(&mut vec);
        assert_eq!(vec, vec![19, 37, 46, 57, 64, 73, 75, 91]);
    }
	#[test]
    fn test_sort_2() {
        let mut vec = vec![1];
        sort(&mut vec);
        assert_eq!(vec, vec![1]);
    }
	#[test]
    fn test_sort_3() {
        let mut vec = vec![99, 88, 77, 66, 55, 44, 33, 22, 11];
        sort(&mut vec);
        assert_eq!(vec, vec![11, 22, 33, 44, 55, 66, 77, 88, 99]);
    }
}