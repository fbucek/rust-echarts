/// Echart is wrapper arount javascript library echarts
/// 
pub mod iso;
pub mod echarts;
pub use echarts::*;

#[macro_use]
extern crate serde_derive;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
