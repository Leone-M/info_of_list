use info_of_list::list_methods;

fn main() {
    let vec: Vec<i32> = vec![1, 2, 3, 4, 5, 6];
    let mean: f32 = list_methods::mean(&vec);
    let median_element: i32 = list_methods::median(&vec);
    let mode_element: i32 = list_methods::mode(&vec);
    print!("Mean: {}\n", mean);
    print!("Median: {}\n", median_element);
    print!("Mode: {}\n", mode_element);
}
