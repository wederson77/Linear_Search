mod linear_search;
use linear_search::linear_search;

fn main() {

    let nums = vec![1,2,3,4,5,6];
    let target = 4;
    let result = linear_search(&nums, target);

    if result == -1{
        println!("O numero {} não foi encontrado no vetor.", target);
    }else{
        println!("O numero {} foi encontrado no indice {} do vetor.", target, result);
    }
}
