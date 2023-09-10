# README

## Introduce

本仓库是ZJU短学期作业写一个异步运行时

## Feature

实现了单线程的异步运行时

#### 基于单线程的性能优化

使用lazy_static全局变量，确保预先初始化

增加drop方法，确保任务完成

## How to use

本runtime库是一个单文件库，需要添加相关依赖后，将项目文件runtime.rs添加到项目目录，根据结构引用

## Test

直接使用cargo run即可测试

测试代码如下

- ![future.png](https://img1.imgtp.com/2023/09/11/dAGrIvfJ.png)

测试结果如下，顺利运行

- ![result.png](https://img1.imgtp.com/2023/09/11/SVVDr3xj.png)

可见在block_on后主任务运行，在spawn后，将任务添加到任务队列，随后执行，直接使用.await可以异步执行异步任务



## Beg Score

对于没学Os的孩子真的好难好难的，球球助教给好分
