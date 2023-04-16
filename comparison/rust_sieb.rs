fn basic_sieve() {

}

fn improved_sieve(n: i64) {
    let mut bool_array = vec![true; n as usize];
    bool_array[0] = false;
    bool_array[1] = false;

    let isqrt = (n as f64).sqrt().floor() as i64;
    for number in 2..isqrt {
        if bool_array[number as usize]{
            for i in (number*number..n).step_by(number as usize) {
                bool_array[i as usize] = false
            }
        }
    }
    let mut output_array = Vec::new();
    for i in 0..n{
        if bool_array[i as usize]{
            output_array.push(i)
        }
    }
}

fn main() {
}