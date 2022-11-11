// fn main() {
//     let arr1 = [1,2,3,4,5,6,7];
//     let arr2 = [2,3,4];
//     let arr3 = [2,4,3];
    
//     // True
//     println!("{}", check_arr_1(&arr1, &arr2));
//     println!("{}", check_arr_2(&arr1, &arr2));
//     println!("{}", check_arr_3(&arr1, &arr2));

//     //False
//     println!("{}", check_arr_1(&arr1, &arr3));
//     println!("{}", check_arr_2(&arr1, &arr3));
//     println!("{}", check_arr_3(&arr1, &arr3));

// }

/*
Bài tập 1: Cho 2 mảng có các phần tử là số nguyên dương, kiểm tra mảng này có phải là mảng
con của mảng kia không ?(yêu cầu đúng thứ tự của các phần tử)
let org_arr = [1, 2,3,5,6,8, 10, 11];
let sub_arr = [6,8,10];
*/
// Cách 1
// fn check_arr_1(arr1: &[i32], arr2: &[i32]) -> bool {
//     let len_1 = arr1.len();
//     let len_2 = arr2.len();
//     let mut res = false;

//     if len_1 < len_2 {
//         return false;
//     }

//     for (index_1, item_1) in arr1.iter().take(len_1 - len_2 + 1).enumerate() {
//         if &arr2[0] == item_1 {
//             res = true;
//             for i in 1..len_2 {
//                 if arr1[index_1 + i] != arr2[i] {
//                     res = false;
//                     break;
//                 }
//             }
//             if res {
//                 return res;
//             }
//         }
//     }
//     res
// }

// Cách 2
// fn check_arr_2(arr1: &[i32], arr2: &[i32]) -> bool {
//     if arr2.len() > arr1.len() {
//         return false;
//     }
//     for i in arr1.windows(arr2.len()) {
//         if i == arr2 {
//             return true;
//         }
//     }
//     false
// }

// Cách 3
// fn check_arr_3(arr1: &[i32], arr2: &[i32]) -> bool {
//     arr2.is_empty() || arr1.windows(arr2.len()).any(|v| arr2 == v)
// }

/*
Bài 2: Cho 1 chuỗi ký tự, nhập 1 ký tự từ bàn phím trả về số lần xuất hiện của từ đó
trong chuỗi đã cho, và chuỗi không chứa ký tự nhập từ bàn phím. Lưu ý: khong
phân biệt viết hoa, viết thường
Ví dụ: let input = “adbcdaDd”.
- Nhập s = ‘a’ => in ra kết quả : 2, “dbcdDd”
- Nhập s = ‘d’ => in ra kết quả : 4, “abca”
 */

use std::io;

fn main(){
    println!("Nhập chuỗi ký tự đầu vào: ");
    let input = input_from_keyboard();
    println!("Nhập 1 ký tự cần tìm: ");
    let search_char = input_from_keyboard();
    let (count, res) = handle_input_2(&input, &search_char);
    println!("Số lần ký tự \"{search_char}\" xuất hiện trong chuỗi \"{input}\" là: {count} lần");
    println!("Chuỗi sau khi loai bỏ ký tự \"{search_char}\"là: {res}");
}

fn input_from_keyboard() -> String{
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Input error");
    input.pop();
    input.pop();
    input
}

// Cách 1
fn handle_input(input: &String, search_char: &String) -> (u32, String){
    let mut count:u32 = 0;
    let mut res = String::new();
    let new_search_char = search_char.to_lowercase().pop().unwrap();

    for i in input.chars(){
        if i.to_ascii_lowercase() == new_search_char {
            count += 1;
        } else {
            res.push(i);
        }
    }
    (count, res)
}

// Cách 2
fn handle_input_2(input: &String, search_char: &str) -> (usize, String){
    let new_char = search_char.to_lowercase().pop().unwrap();
    let res:Vec<&str> = input.split(&[new_char, new_char.to_ascii_uppercase()]).collect();
    let res = res.join("");
    let count = input.len() - res.len();
    (count, res)
}