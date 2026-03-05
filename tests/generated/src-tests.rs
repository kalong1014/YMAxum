//! src-tests 测试套件
//! 为 src 模块生成的测试套件

#[cfg(test)]
mod tests {
    use super::*;

    /// 测试 add 函数
    #[test]
    fn test_add() {
        assert_eq!(add(1, 2), add(1, 2));
    }

}
