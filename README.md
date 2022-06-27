# cnosdb-isipho
   用RUST来实现时序数据库cnosdb-2.0版本

## cnosdb-isipho roadmap

从更远的角度考虑，为了cnosdb拥有更好的安全、性能指标，计划使用Rust语言来开发cnosdb-isipho的版本.
设计目标如下
1. 扩展性，时间序列膨胀（理论无上限 ）  支持横/纵向 可扩展；
2. 面向多租设计  提供更多的配置参数  能够提供资源quota配置；
3. 存储性能和成本 高性能io，run to complete调度模型，支持使用对象存储进行分级存储；
4. 云原生 面向容器  内存和cpu  核数可配置  减少使用共享内存；
5. 查询使用 arrow datafusion 实现query 利用它的矢量执行引擎，执行复杂的sql；
6. cdc  wal 可以提供订阅和分发到其他节点；
7. 与其他数据生态系统相结合  支持导入/导出parquet文件 上游对接消息系统mqtt/kafka；
8. 提供更多可配置项 用来适配不同的场景比如工控，能处理散列数据 和均匀的时序点密集型时序的点；
   
在重新设计存储tsdb的过程中我们尽可能去解决当前时序数据库面临的一系列问题，形成一套完整的时序数据解决方案及时序生态系统（TSDB ClOUD + ECOSYSTEM ）

| title | content | time |
| :---: | :---: | :---: |
| cnosdb-isipho存储引擎基本框架 | 基本的read/write/grpc | 2022-06 |
| 查询引擎和索引 对接tskv | 单机版cnosdb-isipho | 2022-12 |
| cnosdb-isipho cluster | cnosdb-isipho的集群版 |  |
| 云原生和多租户适配 | 多个云厂商的上架，适配 |  |
| ecosystem | 生态系统开发，上下游软件生态代码社区贡献 |  |
