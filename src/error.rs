use glutin;

error_chain! {

    links {
    }

    foreign_links {
        GlutinCreation(glutin::CreationError);
        GlutinContext(glutin::ContextError);
    }
}

pub trait OptErr<T> {
    fn ok(self) -> Result<T>;
}

impl<T> OptErr<T> for Option<T> {
    fn ok(self) -> Result<T> {
        self.ok_or("Missing value".into())
    }
}

