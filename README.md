//模式1： cargo run -- -i <目录1> <目录2>.... <正则>    可以同时搜索多个path,并输出一共多少个匹配项，并去重排序

//模式2： cargo run -- -v <目录>  <正则>  ，并去重排序，并输出所有的遍历文件

//模式3： cargo run -- -z <目录> <正则1>...<正则n>   可以同时匹配多个正则,并输出一共多少个匹配项，并去重排序

//同时，命令行可以彩色输出，一些语句进行了彩色处理

//同时，将代码重构

//通过tracing输出日志，但有奇怪的环境错误

最终，完成了进阶的所有要求
