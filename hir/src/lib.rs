#[mlir(spmd=true, saneql=true)]

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // foreach 0..10 {
 
        // };
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
