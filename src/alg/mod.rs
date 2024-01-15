// Copyright (c) 2023-2024 Liu Qiang <liulcsy@qq.com> In Wuhan, China
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//I have submitted the leetcode for approval
//https://leetcode.cn/problems/find-peak-element/description/
pub fn find_peak_element(nums:Vec<i32>)-> i32 {
    let mut left = 0;
    let n = nums.len();
    println!("n:{}",n);
    let mut right = n -1;
    while left <= right {
        let mid = (left + right)/ 2;
        println!("mid:{}",mid);
        if mid + 1<n && nums[mid] < nums[mid + 1] {
            left = mid + 1;
            println!("left:{}",left);
        }
        else if (mid as isize) - 1 >= 0 && nums[mid] < nums[mid - 1]{
            right = mid -1;
            println!("right:{}",right);
        } else {
            return mid.try_into().unwrap();
        }
    }
    return -1;
}
