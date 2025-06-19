# House Buying

## Running egui app

Just build the app and run it. You will need [Rust](https://www.rust-lang.org/).

If you are not familiar with it, install it and just run 

```bash
cargo run -r
```

in the root of this repository.

Parameters are changeable in the app itself.

## Running Python pipeline

Configure `data/config/config.toml` according to your needs, and
using as reference the `example_config.toml` file in the same folder.

Then run

```bash
python main.py
```

in the root of the repo, keeping in mind the dependencies
specified in pyproject.toml.

Outputs will be spilled into `data/outputs`.
