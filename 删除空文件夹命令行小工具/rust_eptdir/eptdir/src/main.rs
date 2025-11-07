// std::env - 环境变量和命令行参数
// std::path - 路径处理（PathBuf 等）
use std::path::PathBuf;
use std::env;

// 导入库模块中的函数
use eptdir::clean_directory;

fn main() {
    // 获取当前程序启动时转入的命令行参数。
    let args: Vec<String> = env::args().collect();
    // println!("{:?}", args);["target\\debug\\eptdir.exe", "D:\\桌面\\草稿"]

    // 如果命令行有参数
    let target_dirs: Vec<PathBuf> = if args.len() > 1 {
    // args[0] 通常是程序的名称，因此从 args[1] 开始就是用户传入的路径参数
    // 闭包 是匿名的，可以捕获和使用定义它时所在作用域中的变量。
    // 函数 是命名的，不能直接捕获外部变量，除非通过传递参数的方式。
    // |s|闭包参数定义,配合map(),把迭代进来的每一个参数(&String类型)转换路径类型
        args[1..].iter().map(|s| PathBuf::from(s)).collect()
    } else {
        // env::current_dir() 获取当前工作目录
        // env"程序运行时的环境接口",用来获取或修改环境信息,expect() 如果出错就打印消息并终止程序
        // vec![] 宏创建一个包含单个元素的向量
        vec![env::current_dir().expect("无法获取当前目录")]
        //vec![]类似java创建数组:
        //List<String> list = new ArrayList<>();
        //list.add("hello");

    };

    //检查目录是否存在
    // .len()，它返回的是向量中元素的个数（有几个路径）
    if target_dirs.len() == 1 {
        println!("正在清理 1 个目录...");
    } else {
        println!("正在清理 {} 个目录...", target_dirs.len());
    }

    // 用于跟踪处理结果
    let mut success_count = 0;
    let mut error_count = 0;

    //遍历所有目标目录,enumerate() 方法返回 (索引, 值) 的元组
    for (index, target_dir) in target_dirs.iter().enumerate() {
        // 如果有多个目录，显示当前处理的目录编号
        if target_dirs.len() > 1 {
            println!("========================================");
            println!("处理目录 {}/{}: {}", index + 1, target_dirs.len(), target_dir.display());
            println!("========================================");
        }
        
        // 调用清理函数处理当前目录，match 表达式处理可能的错误
        match clean_directory(target_dir) {
            // Ok 分支：清理成功
            Ok(_) => {
                success_count += 1;
            }
            // Err 分支：清理失败
            Err(e) => {
                error_count += 1;
                eprintln!("清理目录时出错: {} - {}", target_dir.display(), e);
                // 继续处理下一个目录，不退出程序
            }
        }
    }

    //显示最终结果
    println!();
    println!("========================================");
    println!("清理完成！");
    println!("成功: {} 个目录", success_count);
    if error_count > 0 {
        println!("失败: {} 个目录", error_count);
    }
    println!("========================================");
    // 如果有错误，以错误码退出
    if error_count > 0 {
        std::process::exit(1);
    }
}
