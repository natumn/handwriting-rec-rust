extern crate serde;
extern crate serde_pickle;
use std::path::path;
use std::fs::File;

struct Key_files {
    train_img: String;
    train_label: String;
    test_img: String;
    test_label: String;
    dataset_dir: String;
    save_file: ; String;
  
}

trait MNIST {
  fn load_mnist(&self, normalize: bool, flatten: bool, one_hot_lobel: bool) -> Vec<String>;
}

impl MNIST for Key_files {
    //TODO: mnist
  fn load_mnist(&self, normalize: bool, flatten: bool, one_hot_lobel: bool) -> Vec<String> {
    let dataset_dir = 
    let mut dataset = serde_pickle::value_from_reader(dataset_dir+"/mnist.pkl");
    if nomalize  {
      for key in ("train_img", "test_img") {
        dataset[key] = dataset[key] as f64;
	dataset[key] /= 255.0;
      }
    }

    if one_hot_label {
      dataset["train_label"] = change_hot_label(dataset["train_label"]);
      dataset["test_label"] = change_hot_label(dataset["test_label"]);
    }

    if !faltten  {
      for key in ("train_img, test_img") {
        //reshape(-1,1,28,28)
        //dataset[key] = dataset[key];
      }
    }
  }
}
