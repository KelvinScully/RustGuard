fn main() {
    let mut is_closing:bool = false;

    let mut x = 0;
    while !is_closing {
        if x > 1000 {
            is_closing = true;
            break;
        }

        x = x + 1;
    }
}
