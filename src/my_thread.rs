pub struct Callback {
    callback: Box<dyn Fn()>,
}

impl Callback {
    pub fn new<F>(a: F) -> Self
    where
        F: 'static + Fn(),
    {
        Callback {
            callback: Box::new(a),
        }
    }

    pub fn call(&self) {
        (self.callback)();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let callback = || {
            println!("hello!");
        };
        let Object = Callback::new(callback);

        Object.call();
    }
}
