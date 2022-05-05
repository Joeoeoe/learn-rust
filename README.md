# 笔记

## 1.Getting Started
* rustc main.rs可进行单个文件的编译，然后执行
* 但是项目一般使用cargo，类似于nodejs的npm
* cargo new [project-name]: 创建项目
* cargo run：编译代码并执行项目
* cargo build：构建项目。添加--release可构建release项目。

## 3.Common Programming Concepts
* rust是expression-based的语言，expression与statement在rust中需要区分。Statement是执行操作的指令，不返回值；expressions执行后返回值：调用function是一个expression、调用宏是一个expression，scope block是一个expression。expression可以是statement的一部分。注意，expression末尾没有分号，如果加上分号，就将其变成了statement。
```rust
fn main() {
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);
}
```
注意此例子，`x+1`末尾是没有分号的，这是一个expression，才得以把值赋给y。若加上分号，编译时便会报错。