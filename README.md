# glotus

这是一个自己在学习 opengl 时练手用的封装库，简单地封装了一些 opengl 的功能。

## utils

```shell
# 工作区中新建项目
cargo init --lib project_lib
cargo init --bin project_bin

# 运行项目
cargo run -p <project_name>

# 简单地绘制平面
cargo run -p case_01_plane

# 绘制有贴图的立方体
cargo run -p case_02_cube

# 绘制phong光照模型
cargo run -p case_03_phong

# 移动光源下的渲染
cargo run -p case_04_moving_light

# 从obj文件加载模型
cargo run -p case_05_load_mesh

# 显示深度测试的结果
cargo run -p case_05_depth_test

# 模板测试实现的外边缘线
cargo run -p case_07_outline
```
