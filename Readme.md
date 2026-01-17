# MISTAKE cli

## version 0.1.0

尽量减少使用轮子。
采用自己的方式去写。

### 分析 3D 场景图

通过传递一个csv文件，解析当前场景图，根据场景图，分析目前一些资产瓶颈。

mtkcli -i input.csv [-o output.html]

在终端展示对应的图表结构

#### 2026-1-16

happy path

任务

- 成功读取csv文件
- 显示csv数据

#### 20205-1-17

任务
改造点

- 将String类型转换为PathBuf
- 将unwarp部分转换为更好的panic处理
  学习点

sumtype productType Option Pattern Matching
