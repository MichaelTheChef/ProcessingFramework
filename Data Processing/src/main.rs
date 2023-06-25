struct DataOperation {
    name: String,
    operation: Box<dyn Fn(Vec<u32>) -> Vec<u32>>,
}

struct DataProcessingFramework {
    operations: Vec<DataOperation>,
}

impl DataProcessingFramework {
    fn new() -> DataProcessingFramework {
        DataProcessingFramework {
            operations: Vec::new(),
        }
    }

    fn register_operation<F>(&mut self, name: &str, operation: F)
    where
        F: 'static + Fn(Vec<u32>) -> Vec<u32>,
    {
        let data_op = DataOperation {
            name: String::from(name),
            operation: Box::new(operation),
        };
        self.operations.push(data_op);
    }

    fn process_data(&self, data: Vec<u32>) -> Vec<u32> {
        let mut result = data;
        for operation in &self.operations {
            result = (operation.operation)(result);
        }
        result
    }
}

fn main() {
    let mut framework = DataProcessingFramework::new();

    framework.register_operation("add_one", |data| {
        data.into_iter().map(|num| num + 1).collect()
    });

    framework.register_operation("multiply_by_two", |data| {
        data.into_iter().map(|num| num * 2).collect()
    });

    framework.register_operation("filter_even", |data| {
        data.into_iter().filter(|num| num % 2 == 0).collect()
    });

    framework.register_operation("sum", |data| {
        let sum: u32 = data.into_iter().sum();
        vec![sum]
    });

    let input_data = vec![1, 2, 3, 4, 5];
    let output_data = framework.process_data(input_data);

    println!("Output: {:?}", output_data);
}
