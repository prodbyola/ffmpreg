use ffmpreg::cli::{Args, BatchPipeline, Pipeline, is_batch_pattern, is_directory};

fn main() {
	let args = Args::parse();

	let result = if is_batch_pattern(&args.input) {
		let output_dir = args.output.clone().unwrap_or_else(|| "out".to_string());
		let batch =
			BatchPipeline::new(args.input.clone(), output_dir, args.show, args.transforms.clone());
		batch.run()
	} else if args.output.as_ref().map(|o| is_directory(o)).unwrap_or(false) {
		let output_dir = args.output.clone().unwrap();
		let batch =
			BatchPipeline::new(args.input.clone(), output_dir, args.show, args.transforms.clone());
		batch.run()
	} else {
		let pipeline =
			Pipeline::new(args.input.clone(), args.output.clone(), args.show, args.transforms.clone());
		pipeline.run()
	};

	match result {
		Ok(()) => {
			if !args.show {
				if let Some(output) = &args.output {
					println!("Success: {} -> {}", args.input, output);
				}
			}
		}
		Err(e) => {
			eprintln!("Error: {}", e);
			std::process::exit(1);
		}
	}
}
