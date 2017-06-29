use std::path::path;

struct Key_files {
  train_img: String;
  train_label: String;
  test_img: String;
  test_label: String;
}

let url_base = "http://yann.lecun.com/exdb/mnist/";
let key_files = Key_files {
                  train_img: "train-images-idx3-ubyte.gz",
		  train_label: "train-labels-idx1-ubyte.gz",
		  test_img: "t10k-images-idx3-ubyte.gz",
		  test_label: "t10k-labels-idx1-ubyte.gz"
                };

fn download(file_name: String) {
  let file_path = dataset_dir + "/" + file_name;

}
