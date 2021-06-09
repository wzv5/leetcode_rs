闲着没事刷题，不知道啥时候就放弃了。。

也算是个 rust 项目模板，配合 vscode 刷题还是挺方便的。

安装以下 vscode 扩展：

1. rust-analyzer
2. Native Debug
3. LeetCode

修改 LeetCode 工作目录为 `src/leetcode`，然后就可以愉快的刷题了。

推荐使用 rust-analyzer 内置的运行测试功能，只需在每个题目代码下方加入以下内容：

``` rust
struct Solution;

#[test]
fn test() {
    assert_eq!(...);
}
```

就能在 test 函数上方看到“Run Test”按钮，点击即可自动运行测试。

使用了 dtolnay 大佬的 automod 项目，自动包含所有源文件。但是原版不支持数字开头的文件名，所以进行了魔改。

如果新增题目文件后，rust-analyzer 没有自动包含新增的文件，只需按下 Ctrl+Shift+P，选择“Rust Analyzer: Restart server”，重新加载后即可。
