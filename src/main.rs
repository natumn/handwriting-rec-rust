extern crate ndarray;
use ndarray::prelude::*;

fn get_data() -> (x_test: ,t_test:  ) {
    let x_train,t_train,x_test,t_test = load_mnist(true, true, false);
}

//TODO: Reading weight parameter saved sample_weight.pkl
fn init_network() -> {

}

//TODO: network build
fn predict(network: Vec<f64>, x: Vec<f64>) -> f64 {
    let w1 = Array::

    let a1 = 
}

fn main() {
    let x, t = get_data();
    let mut network = init_network();
    let accuracy_cnt = 0;


}
