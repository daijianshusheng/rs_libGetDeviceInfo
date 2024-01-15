

pub fn find_peak_element(nums:Vec<i32>)-> i32 {
    let mut left = 0;
    let n = nums.len();
    let mut right = n -1;
    while left <= right {
        let mid = (left + right)/ 2;
        //println!("mid:{}",mid);
        if mid + 1<n && nums[mid] < nums[mid + 1] {
            left = mid + 1;
           // println!("left:{}",left);
        }
        else if mid-1 == 0 && nums[mid] < nums[mid - 1] && mid-1 > 0{
            right = mid -1;
            //println!("right:{}",right);
        } else {
            return mid.try_into().unwrap();
        }
    }
    return -1;
}
