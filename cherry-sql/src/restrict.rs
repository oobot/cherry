
pub struct Restrict {

}

pub trait Where {

    fn gt(&self) {

    }

}

impl Where for &str {
    fn gt(&self) {
        todo!()
    }
}