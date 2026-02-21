trait Process {
    // associated function
    fn total();
    // method
    fn start(&self);
}

struct Upload;

impl Process for Upload {
    fn total() {
        println!("Total from trait.");
    }

    fn start(&self) {
        println!("Start from trait.");
    }
}

impl Upload {
    fn total() {
        println!("Total from struct.");
    }

    fn start(&self) {
         println!("Start from struct.");
    }
}

fn main() {
    // Use fully qualified syntax
    <Upload as Process>::total();
    // Total from trait.

    // We can use fully qualified syntax everywhere
    // that we call associated functions or methods.
    // But this is only necessary in some cases
    // when the compiler itself cannot determine what exactly to call.

    Upload::total();
    // Total from struct.

    let upload = Upload;
    // compilation error: "error[E0599]: no method named `total` found for struct `Upload` in the current scope"
    // upload.total();

    // The both are equivalent
    <Upload as Process>::start(&upload);
    // Start from trait.
    Process::start(&upload);
    // Start from trait.

    // The both are equivalent
    Upload::start(&upload);
    // Start from struct.
    upload.start();
    // Start from struct.
}
