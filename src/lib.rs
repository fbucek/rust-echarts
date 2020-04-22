/// Echart is wrapper arount javascript library echarts
/// 
pub mod echarts;

pub use echarts::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
