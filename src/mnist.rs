extern crate serde;
extern crate serde_pickle;
use std::path::path;
use std::fs::File;

struct Key_files {
  train_img: String;
  train_label: String;
  test_img: String;
  test_label: String;
}

trait MNIST {
  fn load_mnist(&self, normalize: bool, flatten: bool, one_hot_lobel: bool) -> Vec<String>;
}

impl MNIST for Key_files {
  fn load_mnist(&self, normalize: bool, flatten: bool, one_hot_lobel: bool) -> Vec<String> {
    let mut dataset = serde_pickle::value_from_reader(dataset_dir+"/mnist.pkl");
    if(nomalize) {
      for key in ("train_img", "test_img") {
        dataset[key] = dataset[key] as f64;
	dataset[key] /= 255.0;
      }
    }

    if(one_hot_label) {
      dataset["train_label"] = change_hot_label(dataset["train_label"]);
      dataset["test_label"] = change_hot_label(dataset["test_label"]);
    }

    if(!faltten) {
      for key in ("train_img, test_img") {
        //reshape(-1,1,28,28)
        //dataset[key] = dataset[key];
      }
    }
  }
}

//TODO: pythonで取得していたpickleファイルからデータ（学習済み）を取得。

let url_base = "http://yann.lecun.com/exdb/mnist/";
let key_files = Key_files {
                  train_img: "train-images-idx3-ubyte.gz",
		  train_label: "train-labels-idx1-ubyte.gz",
		  test_img: "t10k-images-idx3-ubyte.gz",
		  test_label: "t10k-labels-idx1-ubyte.gz"
                };
//絶対パスの取得
let dataset_dir = "";
let save_file = dataset_dir + "/mnist.pkl";

//fn download(file_name: String) {
//  let file_path = dataset_dir + "/" + file_name;
//}

pub fn load_minst(normalize: bool,flatten: bool,one_hot_label: bool){
  let mnist_file = serde-pickle::de::value_form_reader();
}
