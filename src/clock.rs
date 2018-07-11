use memory::Memory;

pub trait Executable {
    fn execute(&mut self, memory: &mut Memory);
}
