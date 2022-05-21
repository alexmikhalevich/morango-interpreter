pub type Value = u16;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum OpCodes {
    LOAD = 0x01,
    WRT = 0x02,
    READ = 0x03,
    ADD = 0x04,
    MULT = 0x05,
    RTN = 0x06,
    TEGT = 0x07,
    TELT = 0x08,
    TEEQ = 0x09,
    GOTO = 0x0A,
    DUP = 0x0B,
    POP = 0x0C,
}

pub trait Visitor<T> {
    fn visit_load(&mut self, ctx: &mut T) -> Result<(), String>
    where
        Self: Sized;
    fn visit_wrt(&mut self, ctx: &mut T) -> Result<(), String>
    where
        Self: Sized;
    fn visit_read(&mut self, ctx: &mut T) -> Result<(), String>
    where
        Self: Sized;
    fn visit_add(&mut self, ctx: &mut T) -> Result<(), String>
    where
        Self: Sized;
    fn visit_mult(&mut self, ctx: &mut T) -> Result<(), String>
    where
        Self: Sized;
    fn visit_rtn(&mut self, ctx: &mut T) -> Result<(), String>
    where
        Self: Sized;
    fn visit_goto(&mut self, ctx: &mut T) -> Result<(), String>
    where
        Self: Sized;
    fn visit_test_eq(&mut self, ctx: &mut T) -> Result<(), String>
    where
        Self: Sized;
    fn visit_test_gt(&mut self, ctx: &mut T) -> Result<(), String>
    where
        Self: Sized;
    fn visit_test_lt(&mut self, ctx: &mut T) -> Result<(), String>
    where
        Self: Sized;
    fn visit_dup(&mut self, ctx: &mut T) -> Result<(), String>
    where
        Self: Sized;
    fn visit_pop(&mut self, ctx: &mut T) -> Result<(), String>
    where
        Self: Sized;
}
